using System.Net;
using System.Reflection;
using Chorizite.ACProtocol;
using Chorizite.ACProtocol.Enums;
using Chorizite.ACProtocol.Messages;
using Chorizite.ACProtocol.Packets;
using Medo.IO.Pcap;

class Program
{
    static void Main(string[] args)
    {
        string? inputFile = args.Length > 0 ? args[0] : null;

        if (inputFile == null || !File.Exists(inputFile))
        {
            Console.Error.WriteLine($"File not found: {inputFile}");
            Environment.Exit(1);
        }

        bool isBinFile = Path.GetExtension(inputFile).Equals(".bin", StringComparison.OrdinalIgnoreCase);

        if (isBinFile)
        {
            if (args.Length != 1)
            {
                Console.Error.WriteLine("Usage for .bin files: dotnet run -- <bin_file>");
                Environment.Exit(1);
            }
            ReadAndPrintBinFile(inputFile);
        }
        else
        {
            if (args.Length != 2)
            {
                Console.Error.WriteLine("Usage for pcap files: dotnet run -- <pcap_file> <output_dir>");
                Console.Error.WriteLine("  <pcap_file>: Path to the pcap file to process");
                Console.Error.WriteLine("  <output_dir>: Directory where .bin files will be written");
                Environment.Exit(1);
            }

            string outputDir = args[1];
            if (!Directory.Exists(outputDir))
            {
                Console.Error.WriteLine($"Output directory not found: {outputDir}");
                Environment.Exit(1);
            }

            ProcessPcapFile(inputFile, outputDir);
        }
    }

    static void ProcessPcapFile(string pcapFile, string outputDir)
    {
        Console.WriteLine($"Processing packet capture: {pcapFile}");
        Console.WriteLine($"Output directory: {outputDir}");

        var messages = new List<ACMessage>();
        var packetId = 0;
        var messageId = 0;

        using (var fileStream = File.OpenRead(pcapFile))
        using (var reader = new PcapReader(fileStream))
        {
            var packetReader = new PacketReader(null!, new MessageReader());

            packetReader.OnC2SPacket += (_, e) => ProcessPacket("C2S", ref packetId, ref messageId, e.Packet, messages);
            packetReader.OnS2CPacket += (_, e) => ProcessPacket("S2C", ref packetId, ref messageId, e.Packet, messages);

            var block = reader.NextBlock(true);
            while (block != null)
            {
                if (block is PcapClassicPacket packet && ExtractPayload(packet) is byte[] payload)
                {
                    var sourceIP = new IPAddress(payload.Skip(26).Take(4).ToArray());
                    var destIP = new IPAddress(payload.Skip(30).Take(4).ToArray());
                    var localIP = IPAddress.Parse("127.0.0.1");

                    if (sourceIP.Equals(localIP))
                        packetReader.HandleC2SPacket(payload.Skip(42).ToArray());
                    else if (destIP.Equals(localIP))
                        packetReader.HandleS2CPacket(payload.Skip(42).ToArray());
                }

                block = reader.NextBlock();
            }
        }

        Console.WriteLine($"PCAP processing complete. Got {messages.Count} messages.");

        for (int i = 0; i < messages.Count; i++)
        {
            var msg = messages[i];
            var dir = msg.MessageDirection.ToString();
            var opCodeHex = $"0x{msg.OpCode:X4}";
            var msgIdHex = GetMessageIdHex(msg.GetType().Name, msg.OpCode);
            var fileName = $"{i:0000}_{dir}_{opCodeHex}_{msgIdHex}_{msg.GetType().Name}.bin";
            var outPath = Path.Combine(outputDir, fileName);

            using (var stream = new FileStream(outPath, FileMode.Create, FileAccess.Write))
            using (var writer = new BinaryWriter(stream))
            {
                msg.Write(writer);
            }
            Console.WriteLine($"Wrote {outPath}");
        }
    }

    static void ProcessPacket(string direction, ref int packetId, ref int messageId, ACPacket packet, List<ACMessage> messages)
    {
        Console.WriteLine($"{direction} Packet {packetId}: {packet.Header.Flags}");
        foreach (var msg in packet.Messages)
        {
            Console.WriteLine($"  Message {messageId}: {msg.GetType().Name}");
            messages.Add(msg);
            messageId++;
        }
        packetId++;
    }

    static byte[]? ExtractPayload(PcapClassicPacket packet)
    {
        var data = packet.GetData();
        if (data.Length < 34) return null;

        var etherType = (data[12] << 8) | data[13];
        return etherType == 0x0800 ? data : null;
    }

    static void ReadAndPrintBinFile(string binFile)
    {
        Console.WriteLine($"Reading binary file: {binFile}");

        try
        {
            var fileName = Path.GetFileNameWithoutExtension(binFile);
            var parts = fileName.Split('_');

            if (parts.Length < 4)
            {
                Console.WriteLine("Could not determine message type from filename format.");
                PrintRawBinary(binFile);
                return;
            }

            var direction = parts[1];
            var opCodeHexStr = parts[2];
            
            string msgIdStr;
            string msgTypeName;
            
            // Handle both old format (4 parts) and new format (5+ parts with message ID)
            if (parts.Length >= 5 && parts[3].StartsWith("0x"))
            {
                msgIdStr = parts[3];
                msgTypeName = string.Join("_", parts.Skip(4));
            }
            else
            {
                msgIdStr = "0x0000"; // Old format, no message ID in filename
                msgTypeName = string.Join("_", parts.Skip(3));
            }

            int opCode = opCodeHexStr.StartsWith("0x")
                ? Convert.ToInt32(opCodeHexStr.Substring(2), 16)
                : int.Parse(opCodeHexStr);

            Console.WriteLine($"File: {fileName}");
            Console.WriteLine($"Index: {parts[0]}");
            Console.WriteLine($"Direction: {direction}");
            Console.WriteLine($"OpCode: {opCodeHexStr}");
            Console.WriteLine($"Message ID: {msgIdStr}");
            Console.WriteLine($"Message Name: {msgTypeName}");

            // For Ordered messages (0xF7B0 and 0xF7B1), the opcode wrapper message is also in the Messages namespace
            // For regular messages, deserialize with the specific message type
            bool isOrdered = opCode == 0xF7B0 || opCode == 0xF7B1;
            
            Type messageType;
            if (isOrdered)
            {
                string orderedTypeName = opCode == 0xF7B0 ? "Ordered_GameEvent" : "Ordered_GameAction";
                messageType = FindMessageType(orderedTypeName);
                if (messageType == null)
                {
                    Console.WriteLine($"Could not find ordered message type: {orderedTypeName}");
                    PrintRawBinary(binFile);
                    return;
                }
            }
            else
            {
                messageType = FindMessageType(msgTypeName);
                if (messageType == null)
                {
                    Console.WriteLine($"Could not find message type: {msgTypeName}");
                    PrintRawBinary(binFile);
                    return;
                }
            }

            var instance = (ACMessage)Activator.CreateInstance(messageType);
            using (var fileStream = File.OpenRead(binFile))
            using (var binaryReader = new BinaryReader(fileStream))
            {
                var readMethod = messageType.GetMethod("Read",
                    BindingFlags.Public | BindingFlags.Instance, null, new[] { typeof(BinaryReader) }, null);

                if (readMethod != null)
                {
                    try
                    {
                        readMethod.Invoke(instance, new object[] { binaryReader });
                        Console.WriteLine($"Message OpCode: 0x{instance.OpCode:X4}");
                        PrintObjectFields(instance, 1);
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine($"Error deserializing: {ex.InnerException?.Message}");
                        PrintRawBinary(binFile);
                    }
                }
                else
                {
                    Console.WriteLine("No Read method found on message type.");
                    PrintRawBinary(binFile);
                }
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"Error reading binary file: {ex.Message}");
        }
    }

    static Type? FindMessageType(string typeName)
    {
        var assembly = typeof(ACMessage).Assembly;
        return assembly.GetTypes()
            .Where(t => t.Namespace?.StartsWith("Chorizite.ACProtocol.Messages") == true &&
                        t.IsSubclassOf(typeof(ACMessage)))
            .FirstOrDefault(t => t.Name.Equals(typeName, StringComparison.OrdinalIgnoreCase));
    }

    static string GetMessageIdHex(string messageName, uint wireOpCode = 0)
    {
        try
        {
            // For Ordered messages, look up in GameEventType
            if (wireOpCode == 0xF7B0 || wireOpCode == 0xF7B1)
            {
                var enumType = typeof(GameEventType);
                var field = enumType.GetField(messageName, BindingFlags.Public | BindingFlags.Static | BindingFlags.IgnoreCase);
                if (field != null)
                {
                    var value = field.GetValue(null);
                    return $"0x{Convert.ToUInt32(value):X4}";
                }
            }

            // For non-Ordered messages, the opcode IS the message ID
            // The message type name typically includes direction info that helps us identify it
            // For now, just return the opcode as the message ID
            return $"0x{wireOpCode:X4}";
        }
        catch { }
        return "0x0000";
    }

    static void PrintRawBinary(string filePath)
    {
        var bytes = File.ReadAllBytes(filePath);
        Console.WriteLine($"Raw binary data length: {bytes.Length} bytes");
        Console.Write("First bytes: ");
        for (int i = 0; i < Math.Min(50, bytes.Length); i++)
        {
            Console.Write($"{bytes[i]:X2} ");
        }
        Console.WriteLine();
    }

    static void PrintObjectFields(object obj, int depth, HashSet<object> visited = null)
    {
        if (obj == null) return;

        visited ??= new HashSet<object>();
        if (visited.Contains(obj)) return;
        visited.Add(obj);

        var type = obj.GetType();
        if (type.Namespace?.StartsWith("System") == true ||
            type.Namespace?.StartsWith("Chorizite.ACProtocol.Enums") == true)
            return;

        var indent = new string(' ', depth * 2);
        var fields = type.GetFields(BindingFlags.Public | BindingFlags.Instance);

        foreach (var field in fields)
        {
            try
            {
                var value = field.GetValue(obj);

                if (value == null)
                {
                    Console.WriteLine($"{indent}{field.Name}: null");
                }
                else if (field.FieldType.IsEnum)
                {
                    Console.WriteLine($"{indent}{field.Name}: {value} (0x{Convert.ToUInt32(value):X})");
                }
                else if (field.FieldType.IsPrimitive || field.FieldType == typeof(string))
                {
                    Console.WriteLine($"{indent}{field.Name}: {value}");
                }
                else if (IsCollectionType(field.FieldType) && value is System.Collections.ICollection collection)
                {
                    PrintCollection(field.Name, collection, indent, depth, visited);
                }
                else
                {
                    Console.WriteLine($"{indent}{field.Name}:");
                    PrintObjectFields(value, depth + 1, visited);
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"{indent}{field.Name}: <Error: {ex.Message}>");
            }
        }
    }

    static bool IsCollectionType(Type fieldType)
    {
        if (!fieldType.IsGenericType) return false;
        var def = fieldType.GetGenericTypeDefinition();
        return def == typeof(List<>) || def == typeof(Dictionary<,>);
    }

    static void PrintCollection(string name, System.Collections.ICollection collection, string indent, int depth, HashSet<object> visited)
    {
        Console.WriteLine($"{indent}{name}: [{collection.Count} items]");
        int itemCount = 0;
        foreach (var item in collection)
        {
            if (itemCount >= 5)
            {
                Console.WriteLine($"{indent}  ... and {collection.Count - 5} more items");
                break;
            }

            if (item != null && !item.GetType().IsPrimitive && item.GetType() != typeof(string))
            {
                Console.WriteLine($"{indent}  [{itemCount}]:");
                PrintObjectFields(item, depth + 2, visited);
            }
            else
            {
                Console.WriteLine($"{indent}  [{itemCount}]: {item}");
            }
            itemCount++;
        }
    }
}
