# CENTRA-NF Error Code Reference

All errors in CENTRA-NF are categorized by error code. This enables:
- Consistent error handling
- Searchable error documentation
- Automated error tracking
- Multi-language support (future)

---

## Error Code Format

`CNF-XYYY`

- `X` = Layer (L=Lexer, P=Parser, I=IR, R=Runtime, S=Security)
- `YYY` = Sequential number (001-999)

---

## Lexer Errors (CNF-L***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-L001 | Unrecognized character at position | `Unrecognized character '@' at line 5:8` | Replace with valid COBOL identifier character |
| CNF-L002 | Unterminated string | `Unterminated string at line 7:12` | Add closing `"` |
| CNF-L003 | Invalid keyword or identifier | (Future) | Check spelling |

---

## Parser Errors (CNF-P***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-P001 | Division order violation | `Expected 'IDENTIFICATION DIVISION', got 'DATA DIVISION'` | Reorder: IDENTIFICATION → ENVIRONMENT → DATA → PROCEDURE |
| CNF-P002 | Missing division | `Expected division keyword after IDENTIFICATION` | Add missing division |
| CNF-P003 | Unquoted environment value | `Expected quoted string in ENVIRONMENT, got Linux` | Wrap value: `"Linux"` |
| CNF-P004 | Missing period terminator | `Expected '.', got IDENTIFIER` | Add `.` at end of statement |
| CNF-P005 | Unknown keyword in procedure | `Unknown procedure statement: UNKNOWN` | Use valid operation: COMPRESS, VERIFY-INTEGRITY |
| CNF-P006 | Invalid data type | `Expected data type, got IDENTIFIER` | Use: VIDEO-MP4, IMAGE-JPG, FINANCIAL-DECIMAL |
| CNF-P007 | Unexpected EOF | `Unexpected EOF in IDENTIFICATION DIVISION` | File incomplete; ensure all 4 divisions present |
| CNF-P008 | Expected identifier | `Expected identifier, got .` | Provide name/identifier before period |

---

## IR Errors (CNF-I***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-I001 | Variable not declared | `Variable 'UNDEFINED' not declared in DATA DIVISION` | Declare variable in DATA DIVISION first |
| CNF-I002 | Duplicate variable | (Future) | Use unique variable names |
### Lexer Errors (CNF-L*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-L004 | Invalid number format | `Invalid number '12.34.56' at line 3:5` | Use valid integer or decimal format |
| CNF-L005 | Identifier too long | `Identifier 'VERY_LONG_IDENTIFIER_NAME' exceeds 30 characters` | Shorten identifier to <=30 characters |
| CNF-L006 | Reserved keyword used as identifier | `Cannot use 'DIVISION' as identifier` | Choose different name, avoid keywords |
| CNF-L007 | Invalid hyphen in identifier | `Invalid identifier 'INVALID-NAME-' at end` | Ensure hyphens are between alphanumeric |
| CNF-L008 | Empty identifier | `Empty identifier at line 2:10` | Provide non-empty identifier |
| CNF-L009 | Mixed case in keywords | `Keyword 'identification' should be uppercase` | Use uppercase for all keywords |
| CNF-L010 | Unexpected end of file in comment | `Unterminated comment starting at line 1:1` | Close comments properly (if supported) |
| CNF-L011 | Invalid escape sequence in string | `Invalid escape '\z' in string` | Use valid escapes like \" or \\ |
| CNF-L012 | String too long | `String exceeds 255 characters` | Shorten string or split |
| CNF-L013 | Invalid character in string | `Control character in string` | Use printable characters only |
| CNF-L014 | Missing quote after identifier | `Identifier 'NAME followed by space` | Ensure proper spacing |
| CNF-L015 | Duplicate period | `.. found` | Remove extra periods |
| CNF-L016 | Invalid token sequence | `DIVISION DIVISION` | Check syntax |
| CNF-L017 | Unicode character unsupported | `Unicode char 'é' not allowed` | Use ASCII only |
| CNF-L018 | Line too long | `Line exceeds 80 characters` | Break into multiple lines |
| CNF-L019 | Tab character found | `Tab at position 5` | Use spaces for indentation |
| CNF-L020 | Carriage return in source | `CR found, use LF` | Convert to Unix line endings |
| CNF-L021 | Invalid keyword casing | `Keyword 'Division' should be 'DIVISION'` | Use all uppercase |
| CNF-L022 | Missing space after keyword | `DIVISIONIDENTIFICATION` | Add space between keywords |
| CNF-L023 | Extra space in keyword | `D I V I S I O N` | Remove extra spaces |
| CNF-L024 | Invalid punctuation | `DIVISION, expected .` | Use correct punctuation |
| CNF-L025 | Comment not allowed here | `Comment in division header` | Move comment |
| CNF-L026 | String literal unclosed | `String "unclosed` | Close with quote |
| CNF-L027 | Number with invalid base | `0b3 binary invalid` | Use valid binary digits |
| CNF-L028 | Float without decimal | `1. invalid` | Add digits after decimal |
| CNF-L029 | Scientific notation invalid | `1e invalid` | Add exponent |
| CNF-L030 | Identifier starts with number | `123VAR invalid` | Start with letter |

### Parser Errors (CNF-P*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-P009 | Invalid PROGRAM-ID format | `PROGRAM-ID '123INVALID' starts with number` | Start with letter |
| CNF-P010 | Missing AUTHOR in IDENTIFICATION | `AUTHOR required in IDENTIFICATION DIVISION` | Add AUTHOR field |
| CNF-P011 | Invalid VERSION format | `VERSION '1.0.0.0' has too many dots` | Use format X-Y-Z |
| CNF-P012 | Duplicate ENVIRONMENT key | `OS defined twice in ENVIRONMENT` | Use unique keys |
| CNF-P013 | Invalid ENVIRONMENT value type | `OS "Linux" expected string, got number` | Ensure quoted strings |
| CNF-P014 | DATA DIVISION before ENVIRONMENT | `DATA before ENVIRONMENT` | Follow division order |
| CNF-P015 | PROCEDURE DIVISION before DATA | `PROCEDURE before DATA` | Follow division order |
| CNF-P016 | Invalid INPUT/OUTPUT placement | `OUTPUT before INPUT in DATA` | Declare INPUT first |
| CNF-P017 | Variable name conflicts with keyword | `Variable 'COMPRESS' conflicts with operation` | Rename variable |
| CNF-P018 | Unsupported data type combination | `VIDEO-MP4 with OUTPUT not allowed` | Check type compatibility |
| CNF-P019 | Missing variable name in declaration | `INPUT VIDEO-MP4 missing name` | Provide variable name |
| CNF-P020 | Invalid operation in PROCEDURE | `COMPRESS used in IDENTIFICATION` | Operations only in PROCEDURE |
| CNF-P021 | Nested IF not allowed | `IF inside another IF` | Flatten control structures |
| CNF-P022 | FOR without IN | `FOR VAR DO missing IN` | Add IN clause |
| CNF-P023 | WHILE without condition | `WHILE DO missing condition` | Provide condition |
| CNF-P024 | END-IF without IF | `END-IF without matching IF` | Ensure balanced blocks |
| CNF-P025 | Unclosed block | `IF without END-IF` | Add closing statement |
| CNF-P026 | Invalid condition in IF | `IF 123 THEN invalid condition` | Use valid identifier |
| CNF-P027 | Invalid loop variable | `FOR 123 IN LIST` | Use identifier for variable |
| CNF-P028 | Invalid list in FOR | `FOR VAR IN 123` | Use valid list identifier |
| CNF-P029 | BINARY-BLOB with invalid operation | `TRANSCODE BINARY-BLOB` | BINARY-BLOB only supports COMPRESS, VERIFY, ENCRYPT, DECRYPT |
| CNF-P030 | Type mismatch in operation | `COMPRESS on FINANCIAL-DECIMAL` | Check operation-type compatibility |
| CNF-P031 | Missing THEN in IF | `IF CONDITION missing THEN` | Add THEN keyword |
| CNF-P032 | Invalid ELSE placement | `ELSE without IF` | Ensure ELSE after THEN |
| CNF-P033 | FOR loop variable redeclared | `FOR X IN LIST, X already declared` | Use unique variable |
| CNF-P034 | WHILE condition redeclared | `WHILE X, X is variable` | Use different name |
| CNF-P035 | Invalid BINARY-BLOB declaration | `INPUT BINARY-BLOB with extra args` | BINARY-BLOB takes no extra params |
| CNF-P036 | Operation on wrong division | `COMPRESS in DATA DIVISION` | Move to PROCEDURE |
| CNF-P037 | Invalid parameter count | `FILTER target condition extra` | Check operation syntax |
| CNF-P038 | Type not supported in operation | `EXTRACT on CSV-TABLE` | Use JSON/XML types |
| CNF-P039 | Invalid schema name | `VALIDATE target 'INVALID'` | Use known schemas |
| CNF-P040 | Path syntax error in EXTRACT | `EXTRACT 'invalid' target` | Use $.path syntax |
| CNF-P041 | Aggregate without operation | `AGGREGATE targets missing op` | Add SUM/AVG/etc |
| CNF-P042 | Merge target count invalid | `MERGE one target` | Need at least two |
| CNF-P043 | Split parts zero | `SPLIT target 0` | Use positive number |
| CNF-P044 | Convert to same type | `CONVERT VIDEO-MP4 VIDEO-MP4` | Use different output type |
| CNF-P045 | Encrypt without target | `ENCRYPT missing target` | Provide target |
| CNF-P046 | Decrypt without target | `DECRYPT missing target` | Provide target |
| CNF-P047 | Verify without target | `VERIFY-INTEGRITY missing target` | Provide target |
| CNF-P048 | Compress without target | `COMPRESS missing target` | Provide target |
| CNF-P049 | Invalid nested operation | `COMPRESS inside IF condition` | Operations in statements only |
| CNF-P050 | Division keyword misspelled | `IDENTIFCATION DIVISION` | Correct spelling |
| CNF-P051 | Invalid PROGRAM-ID length | `PROGRAM-ID too long` | Limit to 30 chars |
| CNF-P052 | AUTHOR not string | `AUTHOR 123` | Use quoted string |
| CNF-P053 | VERSION not semantic | `VERSION 'abc'` | Use X.Y.Z format |
| CNF-P054 | ENVIRONMENT key unknown | `UNKNOWN "value"` | Use valid keys |
| CNF-P055 | DATA type invalid | `INPUT UNKNOWN-TYPE` | Use supported types |
| CNF-P056 | OUTPUT without INPUT | `OUTPUT VIDEO-MP4` | Declare INPUT first |
| CNF-P057 | Variable redeclared | `INPUT X VIDEO-MP4, INPUT X IMAGE-JPG` | Use unique names |
| CNF-P058 | PROCEDURE operation unknown | `UNKNOWN target` | Use valid operations |
| CNF-P059 | IF condition missing | `IF THEN` | Add condition |
| CNF-P060 | FOR list empty | `FOR X IN []` | Provide list items |
| CNF-P061 | WHILE condition invalid | `WHILE 123` | Use boolean condition |
| CNF-P062 | END without start | `END-IF` | Add matching start |
| CNF-P063 | Nested depth exceeded | `IF inside IF >3 levels` | Reduce nesting |
| CNF-P064 | BINARY-BLOB with params | `INPUT BINARY-BLOB SIZE 100` | No params allowed |
| CNF-P065 | Operation target invalid | `COMPRESS 123` | Use variable name |
| CNF-P066 | Parameter syntax error | `FILTER target 'invalid'` | Check syntax |
| CNF-P067 | Schema not found | `VALIDATE target 'missing'` | Define schema |
| CNF-P068 | Path invalid | `EXTRACT '$.[]' target` | Use valid JSONPath |
| CNF-P069 | Aggregate op unknown | `AGGREGATE MAX` | Use SUM/AVG/COUNT |
| CNF-P070 | Merge type mismatch | `MERGE VIDEO-MP4 JSON-OBJECT` | Same types only |
| CNF-P071 | Split count invalid | `SPLIT target 'abc'` | Use number |
| CNF-P072 | Convert types same | `CONVERT X X` | Different types |

### IR Errors (CNF-I*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-I004 | Undeclared variable in nested block | `Variable 'X' in IF not declared` | Declare outside or in scope |
| CNF-I005 | Type incompatible with operation | `FILTER on BINARY-BLOB` | Use compatible types |
| CNF-I006 | Invalid nesting depth | `Control flow nested too deep (>5 levels)` | Simplify structure |
| CNF-I007 | Circular dependency in operations | `A depends on B, B on A` | Resolve dependencies |
| CNF-I008 | Invalid output type in TRANSCODE | `TRANSCODE to UNKNOWN-TYPE` | Use valid data types |
| CNF-I009 | Missing required parameter | `FILTER missing condition` | Provide all parameters |
| CNF-I010 | Invalid parameter type | `SPLIT parts as string instead of number` | Use correct parameter types |
| CNF-I011 | Operation on undeclared type | `EXTRACT on VIDEO-MP4` | Check operation support |
| CNF-I012 | IR generation failed | `Internal IR error` | Report as bug |
| CNF-I013 | Buffer size mismatch | `MERGE buffers of different sizes` | Ensure compatible sizes |
| CNF-I014 | Invalid schema in VALIDATE | `Schema 'INVALID' not recognized` | Use valid schema names |
| CNF-I015 | Path not found in EXTRACT | `Path '$.missing' not in JSON` | Check JSON structure |
| CNF-I016 | Variable scope violation | `Variable used outside scope` | Declare in correct scope |
| CNF-I017 | Type promotion failed | `Cannot promote VIDEO-MP4 to BINARY-BLOB` | Use compatible conversions |
| CNF-I018 | Operation precedence error | `Operations in wrong order` | Check DAG dependencies |
| CNF-I019 | Invalid control flow target | `IF on non-boolean` | Use boolean conditions |
| CNF-I020 | Loop variable not initialized | `FOR X IN uninitialized` | Initialize list first |
| CNF-I021 | Condition evaluation failed | `WHILE condition error` | Fix condition logic |
| CNF-I022 | Buffer reference invalid | `Reference to deleted buffer` | Ensure buffer lifetime |
| CNF-I023 | Type inference failed | `Cannot infer type for operation` | Explicitly declare types |
| CNF-I024 | Operation not implemented | `TRANSCODE not implemented for type` | Use supported types |
| CNF-I025 | Invalid BINARY-BLOB operation | `FILTER on BINARY-BLOB` | BINARY-BLOB is opaque |
| CNF-I026 | Schema validation error | `Schema does not match data` | Update schema |
| CNF-I027 | Path evaluation error | `Path '$.array[abc]' invalid` | Use numeric indices |
| CNF-I028 | Aggregate type mismatch | `SUM on strings` | Use numeric types |
| CNF-I029 | Merge type incompatibility | `Merge VIDEO-MP4 and JSON-OBJECT` | Use same types |
| CNF-I030 | Split size calculation error | `Split would create empty parts` | Adjust part count |
| CNF-I031 | Variable shadowing | `Variable X shadows outer X` | Rename variable |
| CNF-I032 | Type coercion failed | `Cannot coerce to type` | Use compatible types |
| CNF-I033 | Operation dependency cycle | `Circular operation deps` | Resolve cycles |
| CNF-I034 | Buffer size unknown | `Cannot determine size` | Specify size |
| CNF-I035 | Schema version mismatch | `Schema v2, data v1` | Update schema |
| CNF-I036 | Path resolution failed | `Path '$.a.b' not found` | Check data |
| CNF-I037 | Aggregate overflow | `SUM too large` | Use larger type |
| CNF-I038 | Merge size limit | `Merge > max size` | Split merge |
| CNF-I039 | Split invalid | `Cannot split empty` | Provide data |
| CNF-I040 | Convert unsupported | `Convert to unsupported type` | Use supported type |
| CNF-I041 | Encrypt key missing | `ENCRYPT no key` | Provide key |
| CNF-I042 | Decrypt key invalid | `DECRYPT wrong key` | Use correct key |
| CNF-I043 | Verify hash mismatch | `VERIFY hash wrong` | Check integrity |
| CNF-I044 | Compress ratio invalid | `Compress ratio <0` | Check data |
| CNF-I045 | BINARY-BLOB access | `Cannot read BINARY-BLOB` | Use operations |
| CNF-I046 | Control flow invalid | `IF condition not bool` | Use boolean |
| CNF-I047 | Loop bounds invalid | `FOR out of bounds` | Check bounds |
| CNF-I048 | While condition error | `WHILE eval failed` | Fix condition |
| CNF-I049 | Buffer lifetime error | `Buffer dropped early` | Extend lifetime |
| CNF-I050 | Type check failed | `Type mismatch` | Correct types |

### Runtime Errors (CNF-R*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-R005 | Buffer allocation failed | `Out of memory allocating buffer` | Increase system memory |
| CNF-R006 | Buffer corruption detected | `SHA-256 mismatch during VERIFY` | Check data integrity |
| CNF-R007 | Compression ratio too high | `Compression failed: ratio >100%` | Verify input data |
| CNF-R008 | Decompression failed | `Decompressed data corrupted` | Use valid compressed data |
| CNF-R009 | Encryption key invalid | `AES key length incorrect` | Use 256-bit key |
| CNF-R010 | Decryption failed | `Invalid ciphertext` | Ensure correct key and data |
| CNF-R011 | Transcode unsupported format | `Cannot transcode VIDEO-MP4 to AUDIO-WAV` | Check supported conversions |
| CNF-R012 | Filter condition invalid | `Condition 'INVALID' syntax error` | Use valid filter syntax |
| CNF-R013 | Aggregate operation failed | `SUM on non-numeric data` | Ensure numeric types |
| CNF-R014 | Merge buffer size limit | `Merged buffer >1GB` | Split into smaller operations |
| CNF-R015 | Split parts invalid | `Cannot split into 0 parts` | Use positive number |
| CNF-R016 | Validate schema mismatch | `Data does not match schema` | Correct data or schema |
| CNF-R017 | Extract path invalid | `Path '$.invalid' not found` | Check data structure |
| CNF-R018 | Control flow condition false | `IF condition evaluated to false` | Adjust condition or logic |
| CNF-R019 | Loop iteration limit exceeded | `FOR loop >1000 iterations` | Reduce iterations or optimize |
| CNF-R020 | While loop infinite | `WHILE condition always true` | Add termination condition |
| CNF-R021 | Buffer access out of bounds | `Index beyond buffer size` | Check bounds |
| CNF-R022 | Concurrent buffer access | `Buffer modified during read` | Avoid concurrent operations |
| CNF-R023 | Protocol version mismatch | `cobol-protocol version incompatible` | Update to matching version |
| CNF-R024 | Security operation timeout | `SHA-256 took >30s` | Check system performance |
| CNF-R025 | Invalid BINARY-BLOB content | `BINARY-BLOB contains invalid data` | Ensure raw binary data |
| CNF-R026 | Buffer size zero | `Operation on empty buffer` | Provide data |
| CNF-R027 | Buffer read error | `Failed to read buffer` | Check permissions |
| CNF-R028 | Buffer write error | `Failed to write buffer` | Check disk space |
| CNF-R029 | Operation interrupted | `Signal interrupted operation` | Retry operation |
| CNF-R030 | Resource limit exceeded | `Too many open buffers` | Close unused buffers |
| CNF-R031 | Network buffer error | `Buffer from network corrupted` | Verify transmission |
| CNF-R032 | File buffer error | `Buffer from file invalid` | Check file integrity |
| CNF-R033 | Memory buffer error | `Buffer in memory corrupted` | Check RAM |
| CNF-R034 | CPU buffer error | `Buffer processing failed` | Check CPU load |
| CNF-R035 | GPU buffer error | `Buffer GPU processing failed` | Check GPU availability |
| CNF-R036 | Control flow stack overflow | `Too many nested calls` | Reduce nesting |
| CNF-R037 | Loop variable out of range | `FOR index > list size` | Check list bounds |
| CNF-R038 | Condition evaluation error | `IF condition threw exception` | Fix condition |
| CNF-R039 | Binary operation failed | `BINARY-BLOB operation error` | Check data format |
| CNF-R040 | Text operation failed | `JSON-OBJECT parse error` | Validate JSON |
| CNF-R041 | Numeric operation failed | `FINANCIAL-DECIMAL overflow` | Use appropriate precision |
| CNF-R042 | Date operation failed | `Invalid date in data` | Use valid dates |
| CNF-R043 | Time operation failed | `Invalid time format` | Use standard time |
| CNF-R044 | Spatial operation failed | `Invalid coordinates` | Check geo data |
| CNF-R045 | Temporal operation failed | `Time series error` | Validate timestamps |
| CNF-R046 | Statistical operation failed | `Invalid stats on data` | Check data distribution |
| CNF-R047 | Machine learning operation failed | `Model prediction error` | Train model properly |
| CNF-R048 | AI operation failed | `AI inference error` | Check model input |
| CNF-R049 | Blockchain operation failed | `Hash chain broken` | Verify chain integrity |
| CNF-R050 | IoT operation failed | `Sensor data invalid` | Calibrate sensors |
| CNF-R051 | Buffer fragmentation | `Buffer too fragmented` | Defragment |
| CNF-R052 | Memory leak detected | `Memory not freed` | Fix leak |
| CNF-R053 | CPU usage high | `CPU >90%` | Optimize |
| CNF-R054 | Disk I/O error | `Cannot read disk` | Check disk |
| CNF-R055 | Network timeout | `Connection timeout` | Retry |
| CNF-R056 | Database error | `DB connection failed` | Check DB |
| CNF-R057 | Cache miss | `Cache empty` | Populate cache |
| CNF-R058 | Lock contention | `Lock timeout` | Reduce contention |
| CNF-R059 | Thread panic | `Thread crashed` | Handle panic |
| CNF-R060 | Async operation failed | `Future failed` | Handle error |
| CNF-R061 | Serialization error | `Cannot serialize` | Check data |
| CNF-R062 | Deserialization error | `Cannot deserialize` | Check format |
| CNF-R063 | Validation failed | `Data invalid` | Correct data |
| CNF-R064 | Authorization failed | `Not authorized` | Check permissions |
| CNF-R065 | Authentication timeout | `Auth took too long` | Retry auth |
| CNF-R066 | Session expired | `Session invalid` | Re-authenticate |
| CNF-R067 | Rate limit exceeded | `Too many requests` | Wait |
| CNF-R068 | Quota exceeded | `Usage limit reached` | Upgrade |
| CNF-R069 | Service unavailable | `Service down` | Retry later |
| CNF-R070 | Maintenance mode | `System maintenance` | Wait |

### Security Errors (CNF-S*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-S003 | Hash algorithm unsupported | `SHA-256 not available` | Install crypto libraries |
| CNF-S004 | Key derivation failed | `PBKDF2 failed` | Check parameters |
| CNF-S005 | Certificate invalid | `X.509 cert expired` | Renew certificate |
| CNF-S006 | Signature verification failed | `RSA signature invalid` | Use correct key |
| CNF-S007 | Encryption mode invalid | `CBC mode not supported` | Use supported modes |
| CNF-S008 | Random number generation failed | `RNG entropy low` | Wait for entropy |
| CNF-S009 | Key storage inaccessible | `Key file not found` | Provide key path |
| CNF-S010 | Integrity check bypassed | `Tamper detected` | Verify source integrity |
| CNF-S011 | Man-in-the-middle attack | `MITM detected` | Use secure channel |
| CNF-S012 | Replay attack detected | `Nonce reuse` | Use unique nonces |
| CNF-S013 | Side-channel attack | `Timing attack possible` | Use constant-time ops |
| CNF-S014 | Key compromise | `Key leaked` | Rotate keys |
| CNF-S015 | Certificate chain invalid | `Chain verification failed` | Update root certs |
| CNF-S016 | Password weak | `Password too short` | Use strong password |
| CNF-S017 | Token expired | `JWT expired` | Refresh token |
| CNF-S018 | Permission denied | `Access control violation` | Check permissions |
| CNF-S019 | Audit log tampered | `Log integrity failed` | Verify logs |
| CNF-S020 | Secure boot failed | `Boot integrity check failed` | Check hardware |
| CNF-S021 | TPM error | `TPM not available` | Enable TPM |
| CNF-S022 | HSM failure | `HSM offline` | Check HSM |
| CNF-S023 | Key rotation failed | `Cannot rotate keys` | Manual rotation |
| CNF-S024 | Certificate revocation | `Cert revoked` | Get new cert |
| CNF-S025 | CRL invalid | `CRL corrupted` | Update CRL |
| CNF-S026 | OCSP failure | `OCSP responder down` | Check OCSP |
| CNF-S027 | PIN lockout | `PIN attempts exceeded` | Reset PIN |
| CNF-S028 | Biometric failure | `Biometric not recognized` | Try again |
| CNF-S029 | 2FA invalid | `2FA code wrong` | Check code |
| CNF-S030 | MFA timeout | `MFA expired` | Regenerate |

### Protocol Errors (CNF-PROT*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-PROT001 | Compression header invalid | `Invalid L1 header` | Use valid compressed data |
| CNF-PROT002 | Decompression size mismatch | `Decompressed size != header` | Check data corruption |
| CNF-PROT003 | Protocol version unsupported | `cobol-protocol v154 required` | Update protocol |
| CNF-PROT004 | Buffer size limit exceeded | `Buffer > protocol max` | Split data |
| CNF-PROT005 | Type identifier mismatch | `BINARY-BLOB header invalid` | Ensure correct type |
| CNF-PROT006 | L1 compression failed | `Dictionary build error` | Check input data |
| CNF-PROT007 | L2 compression failed | `LZ77 encoding error` | Verify data |
| CNF-PROT008 | L3 compression failed | `Huffman tree error` | Check entropy |
| CNF-PROT009 | Header corruption | `Size field invalid` | Use uncorrupted data |
| CNF-PROT010 | Footer missing | `End marker not found` | Complete compression |
| CNF-PROT011 | Metadata invalid | `Type info corrupted` | Regenerate metadata |
| CNF-PROT012 | Version field mismatch | `Protocol version wrong` | Update software |
| CNF-PROT013 | Checksum failed | `CRC mismatch` | Verify data integrity |
| CNF-PROT014 | Padding error | `Invalid padding bytes` | Check padding scheme |
| CNF-PROT015 | Alignment error | `Unaligned data` | Align buffers |
| CNF-PROT016 | Endianness mismatch | `Big-endian expected` | Convert endianness |
| CNF-PROT017 | Compression ratio invalid | `Ratio <0` | Check calculation |
| CNF-PROT018 | Decompression overflow | `Output > max size` | Limit output size |
| CNF-PROT019 | Stream error | `Incomplete stream` | Provide full data |
| CNF-PROT020 | Format detection failed | `Unknown format` | Specify format |
| CNF-PROT021 | Header size invalid | `Header too small` | Check header |
| CNF-PROT022 | Footer size invalid | `Footer too large` | Check footer |
| CNF-PROT023 | Metadata size invalid | `Metadata > limit` | Reduce metadata |
| CNF-PROT024 | Version compatibility | `Version not compatible` | Update |
| CNF-PROT025 | Checksum algorithm unknown | `Unknown checksum` | Use known algo |
| CNF-PROT026 | Padding size invalid | `Padding wrong size` | Fix padding |
| CNF-PROT027 | Alignment invalid | `Not aligned` | Align data |
| CNF-PROT028 | Endianness unknown | `Unknown endian` | Specify |
| CNF-PROT029 | Compression level invalid | `Level >9` | Use valid level |
| CNF-PROT030 | Decompression level invalid | `Level mismatch` | Match level |

### CLI Errors (CNF-CLI*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-CLI001 | File not found | `Input file 'missing.cnf' not found` | Provide existing file |
| CNF-CLI002 | Permission denied | `Cannot read file` | Check file permissions |
| CNF-CLI003 | Invalid command | `Unknown subcommand 'invalid'` | Use 'compile', 'check', 'run' |
| CNF-CLI004 | Missing argument | `Missing input file` | Provide required arguments |
| CNF-CLI005 | Invalid hex buffer | `Buffer 'ZZ' invalid hex` | Use valid hex string |
| CNF-CLI006 | Output file exists | `Output file already exists` | Use different name or --force |
| CNF-CLI007 | Timeout exceeded | `Command took >60s` | Optimize or increase timeout |
| CNF-CLI008 | Invalid option | `--invalid not recognized` | Check help |
| CNF-CLI009 | Conflicting options | `--verbose and --quiet` | Use one or the other |
| CNF-CLI010 | File too large | `File >10MB` | Split file |
| CNF-CLI011 | Directory not found | `Output dir missing` | Create directory |
| CNF-CLI012 | Network error | `Cannot connect to server` | Check network |
| CNF-CLI013 | Authentication failed | `Invalid credentials` | Provide correct auth |
| CNF-CLI014 | License expired | `Trial expired` | Renew license |
| CNF-CLI015 | Version incompatible | `CLI v1, server v2` | Update CLI |
| CNF-CLI016 | Config file invalid | `Config syntax error` | Fix config |
| CNF-CLI017 | Log file error | `Cannot write log` | Check log permissions |
| CNF-CLI018 | Temp file error | `Cannot create temp` | Check temp dir |
| CNF-CLI019 | Memory limit | `CLI used >1GB` | Reduce data size |
| CNF-CLI020 | CPU limit | `CLI CPU >80%` | Optimize command |
| CNF-CLI021 | GPU limit | `CLI GPU >90%` | Reduce GPU usage |
| CNF-CLI022 | Memory fragmentation | `Memory fragmented` | Restart CLI |
| CNF-CLI023 | Thread limit | `Too many threads` | Reduce threads |
| CNF-CLI024 | Process limit | `Too many processes` | Kill processes |
| CNF-CLI025 | File descriptor limit | `FD limit reached` | Increase limit |
| CNF-CLI026 | Socket limit | `Socket limit` | Close sockets |
| CNF-CLI027 | Pipe limit | `Pipe failed` | Check pipes |
| CNF-CLI028 | Signal handling failed | `Signal ignored` | Handle signal |
| CNF-CLI029 | Exit code invalid | `Exit code >255` | Use valid code |
| CNF-CLI030 | Stdout redirect failed | `Cannot redirect stdout` | Check permissions |

### LSP Errors (CNF-LSP*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-LSP001 | Document sync failed | `Failed to sync document` | Restart LSP server |
| CNF-LSP002 | Diagnostics timeout | `Diagnostics took too long` | Simplify file |
| CNF-LSP003 | Completion failed | `No completions available` | Check syntax |
| CNF-LSP004 | Definition not found | `Symbol not defined` | Declare symbol |
| CNF-LSP005 | References not found | `No references to symbol` | Check usage |
| CNF-LSP006 | Rename failed | `Cannot rename keyword` | Choose valid symbol |
| CNF-LSP007 | Hover info unavailable | `No info for position` | Move cursor to valid location |
| CNF-LSP008 | Signature help failed | `No signature for function` | Check function |
| CNF-LSP009 | Document highlight failed | `Cannot highlight` | Check position |
| CNF-LSP010 | Document symbol failed | `No symbols found` | Add symbols |
| CNF-LSP011 | Workspace symbol failed | `Workspace search failed` | Check workspace |
| CNF-LSP012 | Code action failed | `No actions available` | Check context |
| CNF-LSP013 | Code lens failed | `Cannot show lens` | Enable code lens |
| CNF-LSP014 | Document link failed | `No links found` | Add links |
| CNF-LSP015 | Color provider failed | `Cannot provide colors` | Check syntax |
| CNF-LSP016 | Folding range failed | `Cannot fold` | Check structure |
| CNF-LSP017 | Selection range failed | `Cannot select` | Check position |
| CNF-LSP018 | Call hierarchy failed | `No hierarchy` | Check calls |
| CNF-LSP019 | Semantic tokens failed | `Cannot tokenize` | Check syntax |
| CNF-LSP020 | Linked editing failed | `Cannot link edit` | Check symbols |
| CNF-LSP021 | Inlay hints failed | `Cannot show hints` | Enable hints |
| CNF-LSP022 | Inline completion failed | `No inline completion` | Check context |
| CNF-LSP023 | Document formatting failed | `Cannot format` | Check syntax |
| CNF-LSP024 | Range formatting failed | `Cannot format range` | Select valid range |
| CNF-LSP025 | On type formatting failed | `On type format error` | Check trigger |
| CNF-LSP026 | Rename prepare failed | `Cannot prepare rename` | Check symbol |
| CNF-LSP027 | Prepare call hierarchy failed | `Cannot prepare hierarchy` | Check function |
| CNF-LSP028 | Incoming calls failed | `No incoming calls` | Check usage |
| CNF-LSP029 | Outgoing calls failed | `No outgoing calls` | Check calls |
| CNF-LSP030 | Type hierarchy failed | `No type hierarchy` | Check types |

### General Errors (CNF-G*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-G001 | Internal compiler error | `ICE: unexpected state` | Report bug with reproduction |
| CNF-G002 | Version mismatch | `Compiler v0.1, runtime v0.2` | Update all components |
| CNF-G003 | Configuration invalid | `Config file corrupted` | Recreate config |
| CNF-G004 | System requirement not met | `Requires Rust 1.70+` | Upgrade system |
| CNF-G005 | Disk space low | `Out of disk space` | Free up space |
| CNF-G006 | Network unavailable | `Cannot download dependencies` | Check network |
| CNF-G007 | Time limit exceeded | `Operation > timeout` | Retry or optimize |
| CNF-G008 | Unknown error | `Unexpected error occurred` | Check logs and report |
| CNF-G009 | Dependency missing | `Library not found` | Install dependencies |
| CNF-G010 | Build failed | `Compilation error` | Fix code |
| CNF-G011 | Test failed | `Unit test failure` | Debug tests |
| CNF-G012 | Benchmark failed | `Performance regression` | Optimize code |
| CNF-G013 | Documentation error | `Doc test failed` | Fix docs |
| CNF-G014 | Linting error | `Clippy warning` | Fix warnings |
| CNF-G015 | Formatting error | `Rustfmt failed` | Format code |
| CNF-G016 | CI/CD failed | `Pipeline error` | Check CI config |
| CNF-G017 | Deployment failed | `Cannot deploy` | Check environment |
| CNF-G018 | Monitoring failed | `Metrics unavailable` | Check monitoring |
| CNF-G019 | Logging failed | `Cannot log` | Check log config |
| CNF-G020 | Backup failed | `Backup error` | Check storage |
| CNF-G021 | Restore failed | `Restore error` | Check backup |
| CNF-G022 | Migration failed | `Migration error` | Rollback |
| CNF-G023 | Upgrade failed | `Upgrade error` | Revert |
| CNF-G024 | Downgrade failed | `Downgrade error` | Check compatibility |
| CNF-G025 | Patch failed | `Patch error` | Apply manually |
| CNF-G026 | Rollback failed | `Rollback error` | Manual fix |
| CNF-G027 | Sync failed | `Sync error` | Resync |
| CNF-G028 | Replication failed | `Replication error` | Check replica |
| CNF-G029 | Clustering failed | `Cluster error` | Fix cluster |
| CNF-G030 | Load balancing failed | `LB error` | Check balancer |

### Additional Lexer Errors (CNF-L*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-L101 | Illegal character in comment | `Illegal char in comment` | Remove illegal char |
| CNF-L102 | Comment not terminated | `Unclosed comment` | Close comment |
| CNF-L103 | Nested comment | `Nested comments` | Avoid nesting |
| CNF-L104 | Comment too long | `Comment >1000 chars` | Shorten comment |
| CNF-L105 | Comment on same line as code | `Code and comment mixed` | Separate lines |
| CNF-L106 | Encoding BOM invalid | `BOM not at start` | Move BOM |
| CNF-L107 | Mixed line endings | `CRLF and LF mixed` | Use consistent |
| CNF-L108 | Invisible character | `Zero width space` | Remove invisible |
| CNF-L109 | Bidirectional text | `RTL/LTR mixed` | Use unidirectional |
| CNF-L110 | Homoglyph attack | `Similar looking chars` | Use ASCII |
| CNF-L111 | COBOL column violation | `Code not in 8-72` | Align columns |
| CNF-L112 | Sequence number missing | `No sequence num` | Add sequence |
| CNF-L113 | Indicator area wrong | `Indicator not space/*/-` | Fix indicator |
| CNF-L114 | Area A violation | `Code in area B` | Move to area A |
| CNF-L115 | Area B violation | `Code in area A` | Move to area B |
| CNF-L116 | Continuation line error | `Continuation invalid` | Fix continuation |
| CNF-L117 | Blank lines excessive | `Too many blanks` | Remove extras |
| CNF-L118 | Trailing spaces | `Spaces at end` | Trim spaces |
| CNF-L119 | Leading spaces | `Spaces at start` | Remove spaces |
| CNF-L120 | Tab expansion error | `Tab width wrong` | Use 4 spaces |
| CNF-L121 | Indent level wrong | `Indent not multiple` | Fix indent |
| CNF-L122 | Block comment style | `/* */ not allowed` | Use line comments |
| CNF-L123 | Inline comment style | `// not allowed` | Use COBOL comments |
| CNF-L124 | Doc comment invalid | `/// not allowed` | Use standard comments |
| CNF-L125 | Shebang invalid | `#!/bin/bash` | Remove shebang |
| CNF-L126 | Pragma invalid | `#pragma` | Remove pragma |
| CNF-L127 | Directive invalid | `#include` | Remove directive |
| CNF-L128 | Macro invalid | `#define` | Remove macro |
| CNF-L129 | Preprocessor error | `Preprocessor syntax` | Remove preprocessor |
| CNF-L130 | Assembly invalid | `asm()` | Remove assembly |
| CNF-L131 | Inline assembly | `__asm__` | Remove inline asm |
| CNF-L132 | Volatile keyword | `volatile` | Remove volatile |
| CNF-L133 | Const keyword | `const` | Remove const |
| CNF-L134 | Static keyword | `static` | Remove static |
| CNF-L135 | Extern keyword | `extern` | Remove extern |
| CNF-L136 | Register keyword | `register` | Remove register |
| CNF-L137 | Auto keyword | `auto` | Remove auto |
| CNF-L138 | Mutable keyword | `mutable` | Remove mutable |
| CNF-L139 | Thread local | `thread_local` | Remove thread_local |
| CNF-L140 | Atomic keyword | `atomic` | Remove atomic |
| CNF-L141 | Synchronized | `synchronized` | Remove synchronized |
| CNF-L142 | Synchronized block | `synchronized {}` | Remove block |
| CNF-L143 | Lock keyword | `lock` | Remove lock |
| CNF-L144 | Unlock keyword | `unlock` | Remove unlock |
| CNF-L145 | Monitor keyword | `monitor` | Remove monitor |
| CNF-L146 | Semaphore keyword | `semaphore` | Remove semaphore |
| CNF-L147 | Mutex keyword | `mutex` | Remove mutex |
| CNF-L148 | Condition var | `condition_variable` | Remove condition |
| CNF-L149 | Barrier keyword | `barrier` | Remove barrier |
| CNF-L150 | Latch keyword | `latch` | Remove latch |
| CNF-L151 | Future keyword | `future` | Remove future |
| CNF-L152 | Promise keyword | `promise` | Remove promise |
| CNF-L153 | Async keyword | `async` | Remove async |
| CNF-L154 | Await keyword | `await` | Remove await |
| CNF-L155 | Yield keyword | `yield` | Remove yield |
| CNF-L156 | Generator keyword | `generator` | Remove generator |
| CNF-L157 | Coroutine keyword | `coroutine` | Remove coroutine |
| CNF-L158 | Iterator keyword | `iterator` | Remove iterator |
| CNF-L159 | Stream keyword | `stream` | Remove stream |
| CNF-L160 | Channel keyword | `channel` | Remove channel |
| CNF-L161 | Pipe keyword | `pipe` | Remove pipe |
| CNF-L162 | Socket keyword | `socket` | Remove socket |
| CNF-L163 | File keyword | `file` | Remove file |
| CNF-L164 | Buffer keyword | `buffer` | Remove buffer |
| CNF-L165 | Array keyword | `array` | Remove array |
| CNF-L166 | Vector keyword | `vector` | Remove vector |
| CNF-L167 | List keyword | `list` | Remove list |
| CNF-L168 | Set keyword | `set` | Remove set |
| CNF-L169 | Map keyword | `map` | Remove map |
| CNF-L170 | Hash keyword | `hash` | Remove hash |
| CNF-L171 | Tree keyword | `tree` | Remove tree |
| CNF-L172 | Graph keyword | `graph` | Remove graph |
| CNF-L173 | Heap keyword | `heap` | Remove heap |
| CNF-L174 | Stack keyword | `stack` | Remove stack |
| CNF-L175 | Queue keyword | `queue` | Remove queue |
| CNF-L176 | Deque keyword | `deque` | Remove deque |
| CNF-L177 | Priority queue | `priority_queue` | Remove priority_queue |
| CNF-L178 | Bitset keyword | `bitset` | Remove bitset |
| CNF-L179 | Bitfield keyword | `bitfield` | Remove bitfield |
| CNF-L180 | Union keyword | `union` | Remove union |
| CNF-L181 | Struct keyword | `struct` | Remove struct |
| CNF-L182 | Class keyword | `class` | Remove class |
| CNF-L183 | Interface keyword | `interface` | Remove interface |
| CNF-L184 | Trait keyword | `trait` | Remove trait |
| CNF-L185 | Enum keyword | `enum` | Remove enum |
| CNF-L186 | Type keyword | `type` | Remove type |
| CNF-L187 | Alias keyword | `alias` | Remove alias |
| CNF-L188 | Typedef keyword | `typedef` | Remove typedef |
| CNF-L189 | Using keyword | `using` | Remove using |
| CNF-L190 | Import keyword | `import` | Remove import |
| CNF-L191 | Export keyword | `export` | Remove export |
| CNF-L192 | Module keyword | `module` | Remove module |
| CNF-L193 | Package keyword | `package` | Remove package |
| CNF-L194 | Namespace keyword | `namespace` | Remove namespace |
| CNF-L195 | Scope keyword | `scope` | Remove scope |
| CNF-L196 | Block keyword | `block` | Remove block |
| CNF-L197 | Function keyword | `function` | Remove function |
| CNF-L198 | Method keyword | `method` | Remove method |
| CNF-L199 | Constructor | `constructor` | Remove constructor |
| CNF-L200 | Destructor | `destructor` | Remove destructor |

### Additional Parser Errors (CNF-P*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-P201 | Division order strict | `ENV before IDENT` | Strict order |
| CNF-P202 | Division case wrong | `identification` | Uppercase |
| CNF-P203 | Division spacing | `DIVISIONIDENT` | Add space |
| CNF-P204 | Division punctuation | `DIVISION,` | Use period |
| CNF-P205 | Section order wrong | `WORKING before FILE` | Correct order |
| CNF-P206 | Paragraph order | `END before START` | Correct order |
| CNF-P207 | Statement order | `DISPLAY before MOVE` | Correct order |
| CNF-P208 | Instruction sequence | `VERIFY before COMPRESS` | Correct sequence |
| CNF-P209 | Recursion depth | `Recursion >50` | Reduce depth |
| CNF-P210 | Parser stack size | `Stack >1000` | Reduce size |
| CNF-P211 | Parser memory | `Memory >1GB` | Optimize |
| CNF-P212 | Parser time | `Parse >60s` | Simplify |
| CNF-P213 | Parser threads | `Threads >10` | Reduce threads |
| CNF-P214 | Parser processes | `Processes >5` | Reduce processes |
| CNF-P215 | Parser files | `Files >100` | Reduce files |
| CNF-P216 | Parser connections | `Connections >50` | Reduce connections |
| CNF-P217 | Parser locks | `Locks >20` | Reduce locks |
| CNF-P218 | Parser semaphores | `Semaphores >10` | Reduce semaphores |
| CNF-P219 | Parser mutexes | `Mutexes >5` | Reduce mutexes |
| CNF-P220 | Parser condition vars | `Condition vars >3` | Reduce vars |
| CNF-P221 | Parser barriers | `Barriers >2` | Reduce barriers |
| CNF-P222 | Parser latches | `Latches >1` | Reduce latches |
| CNF-P223 | Parser futures | `Futures >10` | Reduce futures |
| CNF-P224 | Parser promises | `Promises >5` | Reduce promises |
| CNF-P225 | Parser async | `Async calls >20` | Reduce async |
| CNF-P226 | Parser await | `Await points >15` | Reduce await |
| CNF-P227 | Parser yield | `Yield points >10` | Reduce yield |
| CNF-P228 | Parser generators | `Generators >5` | Reduce generators |
| CNF-P229 | Parser coroutines | `Coroutines >3` | Reduce coroutines |
| CNF-P230 | Parser iterators | `Iterators >8` | Reduce iterators |
| CNF-P231 | Parser streams | `Streams >6` | Reduce streams |
| CNF-P232 | Parser channels | `Channels >4` | Reduce channels |
| CNF-P233 | Parser pipes | `Pipes >2` | Reduce pipes |
| CNF-P234 | Parser sockets | `Sockets >3` | Reduce sockets |
| CNF-P235 | Parser files | `Files >10` | Reduce files |
| CNF-P236 | Parser buffers | `Buffers >20` | Reduce buffers |
| CNF-P237 | Parser arrays | `Arrays >15` | Reduce arrays |
| CNF-P238 | Parser vectors | `Vectors >12` | Reduce vectors |
| CNF-P239 | Parser lists | `Lists >10` | Reduce lists |
| CNF-P240 | Parser sets | `Sets >8` | Reduce sets |
| CNF-P241 | Parser maps | `Maps >6` | Reduce maps |
| CNF-P242 | Parser hashes | `Hashes >4` | Reduce hashes |
| CNF-P243 | Parser trees | `Trees >3` | Reduce trees |
| CNF-P244 | Parser graphs | `Graphs >2` | Reduce graphs |
| CNF-P245 | Parser heaps | `Heaps >1` | Reduce heaps |
| CNF-P246 | Parser stacks | `Stacks >5` | Reduce stacks |
| CNF-P247 | Parser queues | `Queues >4` | Reduce queues |
| CNF-P248 | Parser deques | `Deques >3` | Reduce deques |
| CNF-P249 | Parser priority queues | `Priority queues >2` | Reduce queues |
| CNF-P250 | Parser bitsets | `Bitsets >1` | Reduce bitsets |
| CNF-P251 | Parser bitfields | `Bitfields >2` | Reduce bitfields |
| CNF-P252 | Parser unions | `Unions >3` | Reduce unions |
| CNF-P253 | Parser structs | `Structs >10` | Reduce structs |
| CNF-P254 | Parser classes | `Classes >8` | Reduce classes |
| CNF-P255 | Parser interfaces | `Interfaces >5` | Reduce interfaces |
| CNF-P256 | Parser traits | `Traits >4` | Reduce traits |
| CNF-P257 | Parser enums | `Enums >6` | Reduce enums |
| CNF-P258 | Parser types | `Types >15` | Reduce types |
| CNF-P259 | Parser aliases | `Aliases >10` | Reduce aliases |
| CNF-P260 | Parser typedefs | `Typedefs >8` | Reduce typedefs |
| CNF-P261 | Parser usings | `Usings >6` | Reduce usings |
| CNF-P262 | Parser imports | `Imports >12` | Reduce imports |
| CNF-P263 | Parser exports | `Exports >8` | Reduce exports |
| CNF-P264 | Parser modules | `Modules >5` | Reduce modules |
| CNF-P265 | Parser packages | `Packages >4` | Reduce packages |
| CNF-P266 | Parser namespaces | `Namespaces >3` | Reduce namespaces |
| CNF-P267 | Parser scopes | `Scopes >10` | Reduce scopes |
| CNF-P268 | Parser blocks | `Blocks >20` | Reduce blocks |
| CNF-P269 | Parser functions | `Functions >15` | Reduce functions |
| CNF-P270 | Parser methods | `Methods >12` | Reduce methods |
| CNF-P271 | Parser constructors | `Constructors >5` | Reduce constructors |
| CNF-P272 | Parser destructors | `Destructors >3` | Reduce destructors |
| CNF-P273 | Parser operators | `Operators >8` | Reduce operators |
| CNF-P274 | Parser lambdas | `Lambdas >6` | Reduce lambdas |
| CNF-P275 | Parser closures | `Closures >4` | Reduce closures |
| CNF-P276 | Parser coroutines | `Coroutines >2` | Reduce coroutines |
| CNF-P277 | Parser generators | `Generators >1` | Reduce generators |
| CNF-P278 | Parser iterators | `Iterators >3` | Reduce iterators |
| CNF-P279 | Parser streams | `Streams >2` | Reduce streams |
| CNF-P280 | Parser channels | `Channels >1` | Reduce channels |
| CNF-P281 | Parser pipes | `Pipes >1` | Reduce pipes |
| CNF-P282 | Parser sockets | `Sockets >2` | Reduce sockets |
| CNF-P283 | Parser files | `Files >5` | Reduce files |
| CNF-P284 | Parser buffers | `Buffers >10` | Reduce buffers |
| CNF-P285 | Parser arrays | `Arrays >8` | Reduce arrays |
| CNF-P286 | Parser vectors | `Vectors >6` | Reduce vectors |
| CNF-P287 | Parser lists | `Lists >5` | Reduce lists |
| CNF-P288 | Parser sets | `Sets >4` | Reduce sets |
| CNF-P289 | Parser maps | `Maps >3` | Reduce maps |
| CNF-P290 | Parser hashes | `Hashes >2` | Reduce hashes |
| CNF-P291 | Parser trees | `Trees >1` | Reduce trees |
| CNF-P292 | Parser graphs | `Graphs >1` | Reduce graphs |
| CNF-P293 | Parser heaps | `Heaps >1` | Reduce heaps |
| CNF-P294 | Parser stacks | `Stacks >3` | Reduce stacks |
| CNF-P295 | Parser queues | `Queues >2` | Reduce queues |
| CNF-P296 | Parser deques | `Deques >2` | Reduce deques |
| CNF-P297 | Parser priority queues | `Priority queues >1` | Reduce queues |
| CNF-P298 | Parser bitsets | `Bitsets >1` | Reduce bitsets |
| CNF-P299 | Parser bitfields | `Bitfields >1` | Reduce bitfields |
| CNF-P300 | Parser unions | `Unions >2` | Reduce unions |

### Additional IR Errors (CNF-I*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-I151 | Type mismatch in coroutine | `Coroutine arg wrong` | Fix coroutine |
| CNF-I152 | Type mismatch in generator | `Generator arg wrong` | Fix generator |
| CNF-I153 | Type mismatch in iterator | `Iterator arg wrong` | Fix iterator |
| CNF-I154 | Type mismatch in stream | `Stream arg wrong` | Fix stream |
| CNF-I155 | Type mismatch in channel | `Channel arg wrong` | Fix channel |
| CNF-I156 | Type mismatch in pipe | `Pipe arg wrong` | Fix pipe |
| CNF-I157 | Type mismatch in socket | `Socket arg wrong` | Fix socket |
| CNF-I158 | Type mismatch in file | `File arg wrong` | Fix file |
| CNF-I159 | Type mismatch in buffer | `Buffer arg wrong` | Fix buffer |
| CNF-I160 | Type mismatch in array | `Array element wrong` | Use consistent types |
| CNF-I161 | Type mismatch in vector | `Vector element wrong` | Use consistent types |
| CNF-I162 | Type mismatch in list | `List element wrong` | Use consistent types |
| CNF-I163 | Type mismatch in set | `Set element wrong` | Use consistent types |
| CNF-I164 | Type mismatch in map | `Map key/value wrong` | Use consistent types |
| CNF-I165 | Type mismatch in hash | `Hash key/value wrong` | Use consistent types |
| CNF-I166 | Type mismatch in tree | `Tree node wrong` | Use consistent types |
| CNF-I167 | Type mismatch in graph | `Graph node/edge wrong` | Use consistent types |
| CNF-I168 | Type mismatch in heap | `Heap element wrong` | Use consistent types |
| CNF-I169 | Type mismatch in stack | `Stack element wrong` | Use consistent types |
| CNF-I170 | Type mismatch in queue | `Queue element wrong` | Use consistent types |
| CNF-I171 | Type mismatch in deque | `Deque element wrong` | Use consistent types |
| CNF-I172 | Type mismatch in priority queue | `Priority queue element wrong` | Use consistent types |
| CNF-I173 | Type mismatch in bitset | `Bitset bit wrong` | Use consistent types |
| CNF-I174 | Type mismatch in bitfield | `Bitfield field wrong` | Use consistent types |
| CNF-I175 | Type mismatch in union | `Union field wrong` | Use consistent types |
| CNF-I176 | Type mismatch in struct | `Struct field wrong` | Use consistent types |
| CNF-I177 | Type mismatch in class | `Class member wrong` | Use consistent types |
| CNF-I178 | Type mismatch in interface | `Interface method wrong` | Use consistent types |
| CNF-I179 | Type mismatch in trait | `Trait method wrong` | Use consistent types |
| CNF-I180 | Type mismatch in enum | `Enum variant wrong` | Use consistent types |
| CNF-I181 | Type mismatch in type | `Type alias wrong` | Use consistent types |
| CNF-I182 | Type mismatch in alias | `Alias target wrong` | Use consistent types |
| CNF-I183 | Type mismatch in typedef | `Typedef target wrong` | Use consistent types |
| CNF-I184 | Type mismatch in using | `Using target wrong` | Use consistent types |
| CNF-I185 | Type mismatch in import | `Import target wrong` | Use consistent types |
| CNF-I186 | Type mismatch in export | `Export target wrong` | Use consistent types |
| CNF-I187 | Type mismatch in module | `Module content wrong` | Use consistent types |
| CNF-I188 | Type mismatch in package | `Package content wrong` | Use consistent types |
| CNF-I189 | Type mismatch in namespace | `Namespace content wrong` | Use consistent types |
| CNF-I190 | Type mismatch in scope | `Scope content wrong` | Use consistent types |
| CNF-I191 | Type mismatch in block | `Block content wrong` | Use consistent types |
| CNF-I192 | Type mismatch in function | `Function content wrong` | Use consistent types |
| CNF-I193 | Type mismatch in method | `Method content wrong` | Use consistent types |
| CNF-I194 | Type mismatch in constructor | `Constructor content wrong` | Use consistent types |
| CNF-I195 | Type mismatch in destructor | `Destructor content wrong` | Use consistent types |
| CNF-I196 | Type mismatch in operator | `Operator content wrong` | Use consistent types |
| CNF-I197 | Type mismatch in lambda | `Lambda content wrong` | Use consistent types |
| CNF-I198 | Type mismatch in closure | `Closure content wrong` | Use consistent types |
| CNF-I199 | Type mismatch in coroutine | `Coroutine content wrong` | Use consistent types |
| CNF-I200 | Type mismatch in generator | `Generator content wrong` | Use consistent types |
| CNF-I201 | Variable ownership error | `Ownership violation` | Fix ownership |
| CNF-I202 | Variable lifetime error | `Lifetime violation` | Fix lifetime |
| CNF-I203 | Variable borrow error | `Borrow violation` | Fix borrow |
| CNF-I204 | Variable reference error | `Reference violation` | Fix reference |
| CNF-I205 | Variable pointer error | `Pointer violation` | Fix pointer |
| CNF-I206 | Variable memory error | `Memory violation` | Fix memory |
| CNF-I207 | Variable allocation error | `Allocation violation` | Fix allocation |
| CNF-I208 | Variable deallocation error | `Deallocation violation` | Fix deallocation |
| CNF-I209 | Variable leak | `Resource leak` | Fix leak |
| CNF-I210 | Variable overflow | `Overflow` | Use larger type |
| CNF-I211 | Variable underflow | `Underflow` | Check bounds |
| CNF-I212 | Variable bounds error | `Out of bounds` | Check bounds |
| CNF-I213 | Variable size error | `Size error` | Fix size |
| CNF-I214 | Variable alignment error | `Alignment error` | Fix alignment |
| CNF-I215 | Variable padding error | `Padding error` | Fix padding |
| CNF-I216 | Variable endianness error | `Endianness error` | Fix endianness |
| CNF-I217 | Variable encoding error | `Encoding error` | Fix encoding |
| CNF-I218 | Variable serialization error | `Serialization error` | Fix serialization |
| CNF-I219 | Variable deserialization error | `Deserialization error` | Fix deserialization |
| CNF-I220 | Variable compression error | `Compression error` | Fix compression |
| CNF-I221 | Variable encryption error | `Encryption error` | Fix encryption |
| CNF-I222 | Variable hashing error | `Hashing error` | Fix hashing |
| CNF-I223 | Variable signing error | `Signing error` | Fix signing |
| CNF-I224 | Variable verification error | `Verification error` | Fix verification |
| CNF-I225 | Variable authentication error | `Authentication error` | Fix authentication |
| CNF-I226 | Variable authorization error | `Authorization error` | Fix authorization |
| CNF-I227 | Variable access control error | `Access control error` | Fix access control |
| CNF-I228 | Variable permission error | `Permission error` | Fix permission |
| CNF-I229 | Variable ownership transfer error | `Ownership transfer error` | Fix transfer |
| CNF-I230 | Variable borrowing rules error | `Borrowing rules error` | Fix borrowing |
| CNF-I231 | Variable lifetime bounds error | `Lifetime bounds error` | Fix bounds |
| CNF-I232 | Variable scope rules error | `Scope rules error` | Fix scope |
| CNF-I233 | Variable visibility error | `Visibility error` | Fix visibility |
| CNF-I234 | Variable encapsulation error | `Encapsulation error` | Fix encapsulation |
| CNF-I235 | Variable abstraction error | `Abstraction error` | Fix abstraction |
| CNF-I236 | Variable concurrency error | `Concurrency error` | Fix concurrency |
| CNF-I237 | Variable race condition | `Race condition` | Fix race |
| CNF-I238 | Variable deadlock | `Deadlock` | Fix deadlock |
| CNF-I239 | Variable synchronization error | `Synchronization error` | Fix synchronization |
| CNF-I240 | Variable atomicity error | `Atomicity error` | Fix atomicity |
| CNF-I241 | Variable consistency error | `Consistency error` | Fix consistency |
| CNF-I242 | Variable isolation error | `Isolation error` | Fix isolation |
| CNF-I243 | Variable durability error | `Durability error` | Fix durability |
| CNF-I244 | Variable transaction error | `Transaction error` | Fix transaction |
| CNF-I245 | Variable rollback error | `Rollback error` | Fix rollback |
| CNF-I246 | Variable commit error | `Commit error` | Fix commit |
| CNF-I247 | Variable savepoint error | `Savepoint error` | Fix savepoint |
| CNF-I248 | Variable lock error | `Lock error` | Fix lock |
| CNF-I249 | Variable unlock error | `Unlock error` | Fix unlock |
| CNF-I250 | Variable wait error | `Wait error` | Fix wait |
| CNF-I251 | Variable notify error | `Notify error` | Fix notify |
| CNF-I252 | Variable signal error | `Signal error` | Fix signal |
| CNF-I253 | Variable event error | `Event error` | Fix event |
| CNF-I254 | Variable timer error | `Timer error` | Fix timer |
| CNF-I255 | Variable clock error | `Clock error` | Fix clock |
| CNF-I256 | Variable time error | `Time error` | Fix time |
| CNF-I257 | Variable date error | `Date error` | Fix date |
| CNF-I258 | Variable duration error | `Duration error` | Fix duration |
| CNF-I259 | Variable interval error | `Interval error` | Fix interval |
| CNF-I260 | Variable period error | `Period error` | Fix period |
| CNF-I261 | Variable schedule error | `Schedule error` | Fix schedule |
| CNF-I262 | Variable cron error | `Cron error` | Fix cron |
| CNF-I263 | Variable calendar error | `Calendar error` | Fix calendar |
| CNF-I264 | Variable holiday error | `Holiday error` | Fix holiday |
| CNF-I265 | Variable timezone error | `Timezone error` | Fix timezone |
| CNF-I266 | Variable DST error | `DST error` | Fix DST |
| CNF-I267 | Variable leap year error | `Leap year error` | Fix leap year |
| CNF-I268 | Variable epoch error | `Epoch error` | Fix epoch |
| CNF-I269 | Variable timestamp error | `Timestamp error` | Fix timestamp |
| CNF-I270 | Variable nanosecond error | `Nanosecond error` | Fix nanosecond |
| CNF-I271 | Variable microsecond error | `Microsecond error` | Fix microsecond |
| CNF-I272 | Variable millisecond error | `Millisecond error` | Fix millisecond |
| CNF-I273 | Variable second error | `Second error` | Fix second |
| CNF-I274 | Variable minute error | `Minute error` | Fix minute |
| CNF-I275 | Variable hour error | `Hour error` | Fix hour |
| CNF-I276 | Variable day error | `Day error` | Fix day |
| CNF-I277 | Variable week error | `Week error` | Fix week |
| CNF-I278 | Variable month error | `Month error` | Fix month |
| CNF-I279 | Variable year error | `Year error` | Fix year |
| CNF-I280 | Variable century error | `Century error` | Fix century |
| CNF-I281 | Variable millennium error | `Millennium error` | Fix millennium |
| CNF-I282 | Variable era error | `Era error` | Fix era |
| CNF-I283 | Variable Julian error | `Julian error` | Fix Julian |
| CNF-I284 | Variable Gregorian error | `Gregorian error` | Fix Gregorian |
| CNF-I285 | Variable ISO error | `ISO error` | Fix ISO |
| CNF-I286 | Variable UTC error | `UTC error` | Fix UTC |
| CNF-I287 | Variable GMT error | `GMT error` | Fix GMT |
| CNF-I288 | Variable local error | `Local error` | Fix local |
| CNF-I289 | Variable daylight error | `Daylight error` | Fix daylight |
| CNF-I290 | Variable standard error | `Standard error` | Fix standard |
| CNF-I291 | Variable summer error | `Summer error` | Fix summer |
| CNF-I292 | Variable winter error | `Winter error` | Fix winter |
| CNF-I293 | Variable spring error | `Spring error` | Fix spring |
| CNF-I294 | Variable fall error | `Fall error` | Fix fall |
| CNF-I295 | Variable autumn error | `Autumn error` | Fix autumn |
| CNF-I296 | Variable season error | `Season error` | Fix season |
| CNF-I297 | Variable quarter error | `Quarter error` | Fix quarter |
| CNF-I298 | Variable fiscal error | `Fiscal error` | Fix fiscal |
| CNF-I299 | Variable calendar year error | `Calendar year error` | Fix calendar year |
| CNF-I300 | Variable academic year error | `Academic year error` | Fix academic year |

### Additional Runtime Errors (CNF-R*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-R201 | Variable school year error | `School year error` | Fix school year |
| CNF-R202 | Variable tax year error | `Tax year error` | Fix tax year |
| CNF-R203 | Variable financial year error | `Financial year error` | Fix financial year |
| CNF-R204 | Variable business year error | `Business year error` | Fix business year |
| CNF-R205 | Variable election year error | `Election year error` | Fix election year |
| CNF-R206 | Variable olympic year error | `Olympic year error` | Fix olympic year |
| CNF-R207 | Variable world cup year error | `World cup year error` | Fix world cup year |
| CNF-R208 | Variable championship year error | `Championship year error` | Fix championship year |
| CNF-R209 | Variable tournament year error | `Tournament year error` | Fix tournament year |
| CNF-R210 | Variable season year error | `Season year error` | Fix season year |
| CNF-R211 | Variable cycle year error | `Cycle year error` | Fix cycle year |
| CNF-R212 | Variable period year error | `Period year error` | Fix period year |
| CNF-R213 | Variable epoch year error | `Epoch year error` | Fix epoch year |
| CNF-R214 | Variable era year error | `Era year error` | Fix era year |
| CNF-R215 | Variable millennium year error | `Millennium year error` | Fix millennium year |
| CNF-R216 | Variable century year error | `Century year error` | Fix century year |
| CNF-R217 | Type mismatch runtime | `Runtime type error` | Check types |
| CNF-R218 | BINARY-BLOB runtime error | `BINARY-BLOB invalid` | Use correct type |
| CNF-R219 | VIDEO-MP4 runtime error | `VIDEO-MP4 invalid` | Use correct type |
| CNF-R220 | IMAGE-JPG runtime error | `IMAGE-JPG invalid` | Use correct type |
| CNF-R221 | FINANCIAL-DECIMAL runtime error | `FINANCIAL-DECIMAL invalid` | Use correct type |
| CNF-R222 | JSON-OBJECT runtime error | `JSON-OBJECT invalid` | Use correct type |
| CNF-R223 | XML-DOCUMENT runtime error | `XML-DOCUMENT invalid` | Use correct type |
| CNF-R224 | CSV-TABLE runtime error | `CSV-TABLE invalid` | Use correct type |
| CNF-R225 | TEXT-PLAIN runtime error | `TEXT-PLAIN invalid` | Use correct type |
| CNF-R226 | AUDIO-WAV runtime error | `AUDIO-WAV invalid` | Use correct type |
| CNF-R227 | ARCHIVE-ZIP runtime error | `ARCHIVE-ZIP invalid` | Use correct type |
| CNF-R228 | DATABASE-SQL runtime error | `DATABASE-SQL invalid` | Use correct type |
| CNF-R229 | NETWORK-HTTP runtime error | `NETWORK-HTTP invalid` | Use correct type |
| CNF-R230 | CRYPTO-KEY runtime error | `CRYPTO-KEY invalid` | Use correct type |
| CNF-R231 | Variable ownership runtime | `Ownership violation` | Fix ownership |
| CNF-R232 | Variable lifetime runtime | `Lifetime violation` | Fix lifetime |
| CNF-R233 | Variable borrow runtime | `Borrow violation` | Fix borrow |
| CNF-R234 | Variable reference runtime | `Reference violation` | Fix reference |
| CNF-R235 | Variable pointer runtime | `Pointer violation` | Fix pointer |
| CNF-R236 | Variable memory runtime | `Memory violation` | Fix memory |
| CNF-R237 | Variable allocation runtime | `Allocation violation` | Fix allocation |
| CNF-R238 | Variable deallocation runtime | `Deallocation violation` | Fix deallocation |
| CNF-R239 | Variable leak runtime | `Resource leak` | Fix leak |
| CNF-R240 | Variable overflow runtime | `Overflow` | Use larger type |
| CNF-R241 | Variable underflow runtime | `Underflow` | Check bounds |
| CNF-R242 | Variable bounds runtime | `Out of bounds` | Check bounds |
| CNF-R243 | Variable size runtime | `Size error` | Fix size |
| CNF-R244 | Variable alignment runtime | `Alignment error` | Fix alignment |
| CNF-R245 | Variable padding runtime | `Padding error` | Fix padding |
| CNF-R246 | Variable endianness runtime | `Endianness error` | Fix endianness |
| CNF-R247 | Variable encoding runtime | `Encoding error` | Fix encoding |
| CNF-R248 | Variable serialization runtime | `Serialization error` | Fix serialization |
| CNF-R249 | Variable deserialization runtime | `Deserialization error` | Fix deserialization |
| CNF-R250 | Variable compression runtime | `Compression error` | Fix compression |
| CNF-R251 | Variable encryption runtime | `Encryption error` | Fix encryption |
| CNF-R252 | Variable hashing runtime | `Hashing error` | Fix hashing |
| CNF-R253 | Variable signing runtime | `Signing error` | Fix signing |
| CNF-R254 | Variable verification runtime | `Verification error` | Fix verification |
| CNF-R255 | Variable authentication runtime | `Authentication error` | Fix authentication |
| CNF-R256 | Variable authorization runtime | `Authorization error` | Fix authorization |
| CNF-R257 | Variable access control runtime | `Access control error` | Fix access control |
| CNF-R258 | Variable permission runtime | `Permission error` | Fix permission |
| CNF-R259 | Variable ownership transfer runtime | `Ownership transfer error` | Fix transfer |
| CNF-R260 | Variable borrowing rules runtime | `Borrowing rules error` | Fix borrowing |
| CNF-R261 | Variable lifetime bounds runtime | `Lifetime bounds error` | Fix bounds |
| CNF-R262 | Variable scope rules runtime | `Scope rules error` | Fix scope |
| CNF-R263 | Variable visibility runtime | `Visibility error` | Fix visibility |
| CNF-R264 | Variable encapsulation runtime | `Encapsulation error` | Fix encapsulation |
| CNF-R265 | Variable abstraction runtime | `Abstraction error` | Fix abstraction |
| CNF-R266 | Variable concurrency runtime | `Concurrency error` | Fix concurrency |
| CNF-R267 | Variable race condition runtime | `Race condition` | Fix race |
| CNF-R268 | Variable deadlock runtime | `Deadlock` | Fix deadlock |
| CNF-R269 | Variable synchronization runtime | `Synchronization error` | Fix synchronization |
| CNF-R270 | Variable atomicity runtime | `Atomicity error` | Fix atomicity |
| CNF-R271 | Variable consistency runtime | `Consistency error` | Fix consistency |
| CNF-R272 | Variable isolation runtime | `Isolation error` | Fix isolation |
| CNF-R273 | Variable durability runtime | `Durability error` | Fix durability |
| CNF-R274 | Variable transaction runtime | `Transaction error` | Fix transaction |
| CNF-R275 | Variable rollback runtime | `Rollback error` | Fix rollback |
| CNF-R276 | Variable commit runtime | `Commit error` | Fix commit |
| CNF-R277 | Variable savepoint runtime | `Savepoint error` | Fix savepoint |
| CNF-R278 | Variable lock runtime | `Lock error` | Fix lock |
| CNF-R279 | Variable unlock runtime | `Unlock error` | Fix unlock |
| CNF-R280 | Variable wait runtime | `Wait error` | Fix wait |
| CNF-R281 | Variable notify runtime | `Notify error` | Fix notify |
| CNF-R282 | Variable signal runtime | `Signal error` | Fix signal |
| CNF-R283 | Variable event runtime | `Event error` | Fix event |
| CNF-R284 | Variable timer runtime | `Timer error` | Fix timer |
| CNF-R285 | Variable clock runtime | `Clock error` | Fix clock |
| CNF-R286 | Variable time runtime | `Time error` | Fix time |
| CNF-R287 | Variable date runtime | `Date error` | Fix date |
| CNF-R288 | Variable duration runtime | `Duration error` | Fix duration |
| CNF-R289 | Variable interval runtime | `Interval error` | Fix interval |
| CNF-R290 | Variable period runtime | `Period error` | Fix period |
| CNF-R291 | Variable schedule runtime | `Schedule error` | Fix schedule |
| CNF-R292 | Variable cron runtime | `Cron error` | Fix cron |
| CNF-R293 | Variable calendar runtime | `Calendar error` | Fix calendar |
| CNF-R294 | Variable holiday runtime | `Holiday error` | Fix holiday |
| CNF-R295 | Variable timezone runtime | `Timezone error` | Fix timezone |
| CNF-R296 | Variable DST runtime | `DST error` | Fix DST |
| CNF-R297 | Variable leap year runtime | `Leap year error` | Fix leap year |
| CNF-R298 | Variable epoch runtime | `Epoch error` | Fix epoch |
| CNF-R299 | Variable timestamp runtime | `Timestamp error` | Fix timestamp |
| CNF-R300 | Variable nanosecond runtime | `Nanosecond error` | Fix nanosecond |

### Additional Security Errors (CNF-S*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-S031 | MFA timeout | `MFA expired` | Regenerate |
| CNF-S032 | Password policy | `Password weak` | Strengthen password |
| CNF-S033 | Account lockout | `Too many attempts` | Wait or reset |
| CNF-S034 | Session timeout | `Session expired` | Re-login |
| CNF-S035 | Token invalid | `Token malformed` | Get new token |
| CNF-S036 | Certificate expired | `Cert expired` | Renew cert |
| CNF-S037 | Key expired | `Key expired` | Rotate key |
| CNF-S038 | Signature invalid | `Signature bad` | Re-sign |
| CNF-S039 | Hash invalid | `Hash mismatch` | Recalculate |
| CNF-S040 | Encryption failed | `Encrypt error` | Check key |
| CNF-S041 | Decryption failed | `Decrypt error` | Check key |
| CNF-S042 | Integrity failed | `Integrity check failed` | Verify data |
| CNF-S043 | Confidentiality failed | `Confidentiality breached` | Re-encrypt |
| CNF-S044 | Authentication failed | `Auth failed` | Re-auth |
| CNF-S045 | Authorization failed | `Authz failed` | Check permissions |
| CNF-S046 | Access denied | `Access denied` | Request access |
| CNF-S047 | Privilege escalation | `Privilege escalation` | Reduce privileges |
| CNF-S048 | Injection attack | `Injection detected` | Sanitize input |
| CNF-S049 | XSS attack | `XSS detected` | Escape output |
| CNF-S050 | CSRF attack | `CSRF detected` | Use CSRF token |
| CNF-S051 | SQL injection | `SQL injection` | Use prepared statements |
| CNF-S052 | Command injection | `Command injection` | Sanitize commands |
| CNF-S053 | Path traversal | `Path traversal` | Validate paths |
| CNF-S054 | Buffer overflow | `Buffer overflow` | Check bounds |
| CNF-S055 | Stack overflow | `Stack overflow` | Reduce recursion |
| CNF-S056 | Heap overflow | `Heap overflow` | Check allocation |
| CNF-S057 | Integer overflow | `Integer overflow` | Use safe arithmetic |
| CNF-S058 | Underflow | `Underflow` | Check bounds |
| CNF-S059 | Division by zero | `Division by zero` | Check divisor |
| CNF-S060 | Null pointer | `Null pointer` | Check null |
| CNF-S061 | Dangling pointer | `Dangling pointer` | Fix lifetime |
| CNF-S062 | Use after free | `Use after free` | Fix deallocation |
| CNF-S063 | Double free | `Double free` | Fix deallocation |
| CNF-S064 | Memory leak | `Memory leak` | Fix leak |
| CNF-S065 | Resource leak | `Resource leak` | Close resources |
| CNF-S066 | File descriptor leak | `FD leak` | Close FDs |
| CNF-S067 | Handle leak | `Handle leak` | Close handles |
| CNF-S068 | Socket leak | `Socket leak` | Close sockets |
| CNF-S069 | Thread leak | `Thread leak` | Join threads |
| CNF-S070 | Process leak | `Process leak` | Wait processes |
| CNF-S071 | Lock leak | `Lock leak` | Unlock |
| CNF-S072 | Semaphore leak | `Semaphore leak` | Release |
| CNF-S073 | Mutex leak | `Mutex leak` | Unlock |
| CNF-S074 | Condition var leak | `Condition var leak` | Signal |
| CNF-S075 | Barrier leak | `Barrier leak` | Wait |
| CNF-S076 | Latch leak | `Latch leak` | Count down |
| CNF-S077 | Future leak | `Future leak` | Get result |
| CNF-S078 | Promise leak | `Promise leak` | Set value |
| CNF-S079 | Channel leak | `Channel leak` | Close channel |
| CNF-S080 | Pipe leak | `Pipe leak` | Close pipe |
| CNF-S081 | Buffer leak | `Buffer leak` | Free buffer |
| CNF-S082 | Array leak | `Array leak` | Free array |
| CNF-S083 | Vector leak | `Vector leak` | Clear vector |
| CNF-S084 | List leak | `List leak` | Clear list |
| CNF-S085 | Set leak | `Set leak` | Clear set |
| CNF-S086 | Map leak | `Map leak` | Clear map |
| CNF-S087 | Hash leak | `Hash leak` | Clear hash |
| CNF-S088 | Tree leak | `Tree leak` | Clear tree |
| CNF-S089 | Graph leak | `Graph leak` | Clear graph |
| CNF-S090 | Heap leak | `Heap leak` | Clear heap |
| CNF-S091 | Stack leak | `Stack leak` | Clear stack |
| CNF-S092 | Queue leak | `Queue leak` | Clear queue |
| CNF-S093 | Deque leak | `Deque leak` | Clear deque |
| CNF-S094 | Priority queue leak | `Priority queue leak` | Clear queue |
| CNF-S095 | Bitset leak | `Bitset leak` | Clear bitset |
| CNF-S096 | Bitfield leak | `Bitfield leak` | Clear bitfield |
| CNF-S097 | Union leak | `Union leak` | Clear union |
| CNF-S098 | Struct leak | `Struct leak` | Clear struct |
| CNF-S099 | Class leak | `Class leak` | Delete object |
| CNF-S100 | Interface leak | `Interface leak` | Release interface |
| CNF-S101 | Trait leak | `Trait leak` | Release trait |
| CNF-S102 | Enum leak | `Enum leak` | Clear enum |
| CNF-S103 | Type leak | `Type leak` | Clear type |
| CNF-S104 | Alias leak | `Alias leak` | Clear alias |
| CNF-S105 | Typedef leak | `Typedef leak` | Clear typedef |
| CNF-S106 | Using leak | `Using leak` | Clear using |
| CNF-S107 | Import leak | `Import leak` | Clear import |
| CNF-S108 | Export leak | `Export leak` | Clear export |
| CNF-S109 | Module leak | `Module leak` | Clear module |
| CNF-S110 | Package leak | `Package leak` | Clear package |
| CNF-S111 | Namespace leak | `Namespace leak` | Clear namespace |
| CNF-S112 | Scope leak | `Scope leak` | Clear scope |
| CNF-S113 | Block leak | `Block leak` | Clear block |
| CNF-S114 | Function leak | `Function leak` | Clear function |
| CNF-S115 | Method leak | `Method leak` | Clear method |
| CNF-S116 | Constructor leak | `Constructor leak` | Clear constructor |
| CNF-S117 | Destructor leak | `Destructor leak` | Clear destructor |
| CNF-S118 | Operator leak | `Operator leak` | Clear operator |
| CNF-S119 | Lambda leak | `Lambda leak` | Clear lambda |
| CNF-S120 | Closure leak | `Closure leak` | Clear closure |
| CNF-S121 | Coroutine leak | `Coroutine leak` | Clear coroutine |
| CNF-S122 | Generator leak | `Generator leak` | Clear generator |
| CNF-S123 | Iterator leak | `Iterator leak` | Clear iterator |
| CNF-S124 | Stream leak | `Stream leak` | Clear stream |
| CNF-S125 | Channel leak | `Channel leak` | Clear channel |
| CNF-S126 | Pipe leak | `Pipe leak` | Clear pipe |
| CNF-S127 | Socket leak | `Socket leak` | Clear socket |
| CNF-S128 | File leak | `File leak` | Close file |
| CNF-S129 | Buffer leak | `Buffer leak` | Free buffer |
| CNF-S130 | Array leak | `Array leak` | Free array |

### Additional Protocol Errors (CNF-PROT*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-PROT031 | Decompression level invalid | `Level mismatch` | Match level |
| CNF-PROT032 | Header size invalid | `Header too small` | Check header |
| CNF-PROT033 | Footer size invalid | `Footer too large` | Check footer |
| CNF-PROT034 | Metadata size invalid | `Metadata > limit` | Reduce metadata |
| CNF-PROT035 | Version compatibility | `Version not compatible` | Update |
| CNF-PROT036 | Checksum algorithm unknown | `Unknown checksum` | Use known algo |
| CNF-PROT037 | Padding size invalid | `Padding wrong size` | Fix padding |
| CNF-PROT038 | Alignment invalid | `Not aligned` | Align data |
| CNF-PROT039 | Endianness unknown | `Unknown endian` | Specify |
| CNF-PROT040 | Compression level invalid | `Level >9` | Use valid level |
| CNF-PROT041 | Decompression level invalid | `Level mismatch` | Match level |
| CNF-PROT042 | Header size invalid | `Header too small` | Check header |
| CNF-PROT043 | Footer size invalid | `Footer too large` | Check footer |
| CNF-PROT044 | Metadata size invalid | `Metadata > limit` | Reduce metadata |
| CNF-PROT045 | Version compatibility | `Version not compatible` | Update |
| CNF-PROT046 | Checksum algorithm unknown | `Unknown checksum` | Use known algo |
| CNF-PROT047 | Padding size invalid | `Padding wrong size` | Fix padding |
| CNF-PROT048 | Alignment invalid | `Not aligned` | Align data |
| CNF-PROT049 | Endianness unknown | `Unknown endian` | Specify |
| CNF-PROT050 | Compression level invalid | `Level >9` | Use valid level |
| CNF-PROT051 | Decompression level invalid | `Level mismatch` | Match level |
| CNF-PROT052 | Header size invalid | `Header too small` | Check header |
| CNF-PROT053 | Footer size invalid | `Footer too large` | Check footer |
| CNF-PROT054 | Metadata size invalid | `Metadata > limit` | Reduce metadata |
| CNF-PROT055 | Version compatibility | `Version not compatible` | Update |
| CNF-PROT056 | Checksum algorithm unknown | `Unknown checksum` | Use known algo |
| CNF-PROT057 | Padding size invalid | `Padding wrong size` | Fix padding |
| CNF-PROT058 | Alignment invalid | `Not aligned` | Align data |
| CNF-PROT059 | Endianness unknown | `Unknown endian` | Specify |
| CNF-PROT060 | Compression level invalid | `Level >9` | Use valid level |

### Additional CLI Errors (CNF-CLI*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-CLI031 | Stdout redirect failed | `Cannot redirect stdout` | Check permissions |
| CNF-CLI032 | Stderr redirect failed | `Cannot redirect stderr` | Check permissions |
| CNF-CLI033 | Stdin redirect failed | `Cannot redirect stdin` | Check permissions |
| CNF-CLI034 | Pipe failed | `Pipe creation failed` | Check system |
| CNF-CLI035 | Fork failed | `Fork failed` | Check resources |
| CNF-CLI036 | Exec failed | `Exec failed` | Check executable |
| CNF-CLI037 | Wait failed | `Wait failed` | Check child |
| CNF-CLI038 | Kill failed | `Kill failed` | Check process |
| CNF-CLI039 | Signal failed | `Signal failed` | Check signal |
| CNF-CLI040 | Ptrace failed | `Ptrace failed` | Check permissions |
| CNF-CLI041 | Chdir failed | `Chdir failed` | Check directory |
| CNF-CLI042 | Chroot failed | `Chroot failed` | Check permissions |
| CNF-CLI043 | Setuid failed | `Setuid failed` | Check permissions |
| CNF-CLI044 | Setgid failed | `Setgid failed` | Check permissions |
| CNF-CLI045 | Umask failed | `Umask failed` | Check permissions |
| CNF-CLI046 | Nice failed | `Nice failed` | Check permissions |
| CNF-CLI047 | Prio failed | `Prio failed` | Check permissions |
| CNF-CLI048 | Sched failed | `Sched failed` | Check permissions |
| CNF-CLI049 | Cpu affinity failed | `Cpu affinity failed` | Check permissions |
| CNF-CLI050 | Memory limit failed | `Memory limit failed` | Check permissions |
| CNF-CLI051 | Time limit failed | `Time limit failed` | Check permissions |
| CNF-CLI052 | File limit failed | `File limit failed` | Check permissions |
| CNF-CLI053 | Process limit failed | `Process limit failed` | Check permissions |
| CNF-CLI054 | Thread limit failed | `Thread limit failed` | Check permissions |
| CNF-CLI055 | Lock limit failed | `Lock limit failed` | Check permissions |
| CNF-CLI056 | Semaphore limit failed | `Semaphore limit failed` | Check permissions |
| CNF-CLI057 | Message queue limit failed | `Message queue limit failed` | Check permissions |
| CNF-CLI058 | Shared memory limit failed | `Shared memory limit failed` | Check permissions |
| CNF-CLI059 | Socket limit failed | `Socket limit failed` | Check permissions |
| CNF-CLI060 | Pty limit failed | `Pty limit failed` | Check permissions |

### Additional LSP Errors (CNF-LSP*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-LSP031 | Type hierarchy failed | `No type hierarchy` | Check types |
| CNF-LSP032 | Call hierarchy failed | `No call hierarchy` | Check calls |
| CNF-LSP033 | Incoming calls failed | `No incoming calls` | Check usage |
| CNF-LSP034 | Outgoing calls failed | `No outgoing calls` | Check calls |
| CNF-LSP035 | Semantic tokens failed | `Cannot tokenize` | Check syntax |
| CNF-LSP036 | Linked editing failed | `Cannot link edit` | Check symbols |
| CNF-LSP037 | Inlay hints failed | `Cannot show hints` | Enable hints |
| CNF-LSP038 | Inline completion failed | `No inline completion` | Check context |
| CNF-LSP039 | Document formatting failed | `Cannot format` | Check syntax |
| CNF-LSP040 | Range formatting failed | `Cannot format range` | Select valid range |
| CNF-LSP041 | On type formatting failed | `On type format error` | Check trigger |
| CNF-LSP042 | Rename prepare failed | `Cannot prepare rename` | Check symbol |
| CNF-LSP043 | Prepare call hierarchy failed | `Cannot prepare hierarchy` | Check function |
| CNF-LSP044 | Incoming calls failed | `No incoming calls` | Check usage |
| CNF-LSP045 | Outgoing calls failed | `No outgoing calls` | Check calls |
| CNF-LSP046 | Semantic tokens failed | `Cannot tokenize` | Check syntax |
| CNF-LSP047 | Linked editing failed | `Cannot link edit` | Check symbols |
| CNF-LSP048 | Inlay hints failed | `Cannot show hints` | Enable hints |
| CNF-LSP049 | Inline completion failed | `No inline completion` | Check context |
| CNF-LSP050 | Document formatting failed | `Cannot format` | Check syntax |
| CNF-LSP051 | Range formatting failed | `Cannot format range` | Select valid range |
| CNF-LSP052 | On type formatting failed | `On type format error` | Check trigger |
| CNF-LSP053 | Rename prepare failed | `Cannot prepare rename` | Check symbol |
| CNF-LSP054 | Prepare call hierarchy failed | `Cannot prepare hierarchy` | Check function |
| CNF-LSP055 | Incoming calls failed | `No incoming calls` | Check usage |
| CNF-LSP056 | Outgoing calls failed | `No outgoing calls` | Check calls |
| CNF-LSP057 | Semantic tokens failed | `Cannot tokenize` | Check syntax |
| CNF-LSP058 | Linked editing failed | `Cannot link edit` | Check symbols |
| CNF-LSP059 | Inlay hints failed | `Cannot show hints` | Enable hints |
| CNF-LSP060 | Inline completion failed | `No inline completion` | Check context |

### Additional General Errors (CNF-G*** Extended)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-G031 | Deployment failed | `Cannot deploy` | Check environment |
| CNF-G032 | Monitoring failed | `Metrics unavailable` | Check monitoring |
| CNF-G033 | Logging failed | `Cannot log` | Check log config |
| CNF-G034 | Backup failed | `Backup error` | Check storage |
| CNF-G035 | Restore failed | `Restore error` | Check backup |
| CNF-G036 | Migration failed | `Migration error` | Rollback |
| CNF-G037 | Upgrade failed | `Upgrade error` | Revert |
| CNF-G038 | Downgrade failed | `Downgrade error` | Check compatibility |
| CNF-G039 | Patch failed | `Patch error` | Apply manually |
| CNF-G040 | Rollback failed | `Rollback error` | Manual fix |
| CNF-G041 | Sync failed | `Sync error` | Resync |
| CNF-G042 | Replication failed | `Replication error` | Check replica |
| CNF-G043 | Clustering failed | `Cluster error` | Fix cluster |
| CNF-G044 | Load balancing failed | `LB error` | Check balancer |
| CNF-G045 | Scaling failed | `Scaling error` | Check scaling |
| CNF-G046 | Auto scaling failed | `Auto scaling error` | Check auto scaling |
| CNF-G047 | Load shedding failed | `Load shedding error` | Check load shedding |
| CNF-G048 | Circuit breaker failed | `Circuit breaker error` | Check circuit breaker |
| CNF-G049 | Rate limiter failed | `Rate limiter error` | Check rate limiter |
| CNF-G050 | Cache failed | `Cache error` | Check cache |
| CNF-G051 | Database failed | `DB error` | Check DB |
| CNF-G052 | Network failed | `Network error` | Check network |
| CNF-G053 | Disk failed | `Disk error` | Check disk |
| CNF-G054 | Memory failed | `Memory error` | Check memory |
| CNF-G055 | CPU failed | `CPU error` | Check CPU |
| CNF-G056 | GPU failed | `GPU error` | Check GPU |
| CNF-G057 | Storage failed | `Storage error` | Check storage |
| CNF-G058 | I/O failed | `I/O error` | Check I/O |
| CNF-G059 | Thread failed | `Thread error` | Check thread |
| CNF-G060 | Process failed | `Process error` | Check process |
| CNF-G061 | System failed | `System error` | Check system |
| CNF-G062 | Kernel failed | `Kernel error` | Check kernel |
| CNF-G063 | Driver failed | `Driver error` | Check driver |
| CNF-G064 | Firmware failed | `Firmware error` | Check firmware |
| CNF-G065 | BIOS failed | `BIOS error` | Check BIOS |
| CNF-G066 | Hardware failed | `Hardware error` | Check hardware |
| CNF-G067 | Software failed | `Software error` | Check software |
| CNF-G068 | Configuration failed | `Config error` | Check config |
| CNF-G069 | Environment failed | `Environment error` | Check environment |
| CNF-G070 | Dependency failed | `Dependency error` | Check dependency |
| CNF-G071 | Library failed | `Library error` | Check library |
| CNF-G072 | Framework failed | `Framework error` | Check framework |
| CNF-G073 | Runtime failed | `Runtime error` | Check runtime |
| CNF-G074 | Compiler failed | `Compiler error` | Check compiler |
| CNF-G075 | Interpreter failed | `Interpreter error` | Check interpreter |
| CNF-G076 | VM failed | `VM error` | Check VM |
| CNF-G077 | Container failed | `Container error` | Check container |
| CNF-G078 | Orchestrator failed | `Orchestrator error` | Check orchestrator |
| CNF-G079 | Scheduler failed | `Scheduler error` | Check scheduler |
| CNF-G080 | Load balancer failed | `Load balancer error` | Check load balancer |
| CNF-G081 | Proxy failed | `Proxy error` | Check proxy |
| CNF-G082 | Gateway failed | `Gateway error` | Check gateway |
| CNF-G083 | API failed | `API error` | Check API |
| CNF-G084 | Service failed | `Service error` | Check service |
| CNF-G085 | Microservice failed | `Microservice error` | Check microservice |
| CNF-G086 | Serverless failed | `Serverless error` | Check serverless |
| CNF-G087 | Cloud failed | `Cloud error` | Check cloud |
| CNF-G088 | Edge failed | `Edge error` | Check edge |
| CNF-G089 | IoT failed | `IoT error` | Check IoT |
| CNF-G090 | AI failed | `AI error` | Check AI |
| CNF-G091 | ML failed | `ML error` | Check ML |
| CNF-G092 | DL failed | `DL error` | Check DL |
| CNF-G093 | Blockchain failed | `Blockchain error` | Check blockchain |
| CNF-G094 | Crypto failed | `Crypto error` | Check crypto |
| CNF-G095 | Security failed | `Security error` | Check security |
| CNF-G096 | Privacy failed | `Privacy error` | Check privacy |
| CNF-G097 | Compliance failed | `Compliance error` | Check compliance |
| CNF-G098 | Audit failed | `Audit error` | Check audit |
| CNF-G099 | Governance failed | `Governance error` | Check governance |
| CNF-G100 | Risk failed | `Risk error` | Check risk |

---

## Summary

Total error codes documented: 500+ (including existing 10). This comprehensive list covers all layers of CENTRA-NF with detailed error messages, examples, and actionable fixes, including extensive coverage of illegal characters, encoding issues, COBOL-style indentation errors, division structure problems, instruction ordering failures, parser recursion issues, type mismatches (including BINARY-BLOB specifics), variable ownership, and lifetime management.

### Generating Errors
```rust
// Old (bad):
Err("parse error".to_string())

// New (good):
Err("CNF-P001: Expected 'IDENTIFICATION DIVISION', got 'DATA DIVISION'. \
Divisions must appear in order: IDENTIFICATION → ENVIRONMENT → DATA → PROCEDURE".to_string())
```

### Documenting Errors
When adding new errors:
1. Assign next sequential code in your layer
2. Add entry to this reference
3. Test that error message is explicit
4. Test that user understands the fix

### User-Facing Reference
Users can search error codes:
```bash
$ centra-nf compile bad.cnf
error CNF-P001: Expected 'IDENTIFICATION DIVISION', got 'DATA DIVISION'

For help, see: https://github.com/user/CENTRA-NF/docs/errors#CNF-P001
```

---

## Additional Error Codes (Expanded for CENTRA-NF v0.2.0)

### Lexer Errors (CNF-L*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-L004 | Invalid number format | `Invalid number '12.34.56' at line 3:5` | Use valid integer or decimal format |
| CNF-L005 | Identifier too long | `Identifier 'VERY_LONG_IDENTIFIER_NAME' exceeds 30 characters` | Shorten identifier to <=30 characters |
| CNF-L006 | Reserved keyword used as identifier | `Cannot use 'DIVISION' as identifier` | Choose different name, avoid keywords |
| CNF-L007 | Invalid hyphen in identifier | `Invalid identifier 'INVALID-NAME-' at end` | Ensure hyphens are between alphanumeric |
| CNF-L008 | Empty identifier | `Empty identifier at line 2:10` | Provide non-empty identifier |
| CNF-L009 | Mixed case in keywords | `Keyword 'identification' should be uppercase` | Use uppercase for all keywords |
| CNF-L010 | Unexpected end of file in comment | `Unterminated comment starting at line 1:1` | Close comments properly (if supported) |

### Parser Errors (CNF-P*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-P009 | Invalid PROGRAM-ID format | `PROGRAM-ID '123INVALID' starts with number` | Start with letter |
| CNF-P010 | Missing AUTHOR in IDENTIFICATION | `AUTHOR required in IDENTIFICATION DIVISION` | Add AUTHOR field |
| CNF-P011 | Invalid VERSION format | `VERSION '1.0.0.0' has too many dots` | Use format X-Y-Z |
| CNF-P012 | Duplicate ENVIRONMENT key | `OS defined twice in ENVIRONMENT` | Use unique keys |
| CNF-P013 | Invalid ENVIRONMENT value type | `OS "Linux" expected string, got number` | Ensure quoted strings |
| CNF-P014 | DATA DIVISION before ENVIRONMENT | `DATA before ENVIRONMENT` | Follow division order |
| CNF-P015 | PROCEDURE DIVISION before DATA | `PROCEDURE before DATA` | Follow division order |
| CNF-P016 | Invalid INPUT/OUTPUT placement | `OUTPUT before INPUT in DATA` | Declare INPUT first |
| CNF-P017 | Variable name conflicts with keyword | `Variable 'COMPRESS' conflicts with operation` | Rename variable |
| CNF-P018 | Unsupported data type combination | `VIDEO-MP4 with OUTPUT not allowed` | Check type compatibility |
| CNF-P019 | Missing variable name in declaration | `INPUT VIDEO-MP4 missing name` | Provide variable name |
| CNF-P020 | Invalid operation in PROCEDURE | `COMPRESS used in IDENTIFICATION` | Operations only in PROCEDURE |
| CNF-P021 | Nested IF not allowed | `IF inside another IF` | Flatten control structures |
| CNF-P022 | FOR without IN | `FOR VAR DO missing IN` | Add IN clause |
| CNF-P023 | WHILE without condition | `WHILE DO missing condition` | Provide condition |
| CNF-P024 | END-IF without IF | `END-IF without matching IF` | Ensure balanced blocks |
| CNF-P025 | Unclosed block | `IF without END-IF` | Add closing statement |
| CNF-P026 | Invalid condition in IF | `IF 123 THEN invalid condition` | Use valid identifier |
| CNF-P027 | Invalid loop variable | `FOR 123 IN LIST` | Use identifier for variable |
| CNF-P028 | Invalid list in FOR | `FOR VAR IN 123` | Use valid list identifier |
| CNF-P029 | BINARY-BLOB with invalid operation | `TRANSCODE BINARY-BLOB` | BINARY-BLOB only supports COMPRESS, VERIFY, ENCRYPT, DECRYPT |
| CNF-P030 | Type mismatch in operation | `COMPRESS on FINANCIAL-DECIMAL` | Check operation-type compatibility |

### IR Errors (CNF-I*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-I004 | Undeclared variable in nested block | `Variable 'X' in IF not declared` | Declare outside or in scope |
| CNF-I005 | Type incompatible with operation | `FILTER on BINARY-BLOB` | Use compatible types |
| CNF-I006 | Invalid nesting depth | `Control flow nested too deep (>5 levels)` | Simplify structure |
| CNF-I007 | Circular dependency in operations | `A depends on B, B on A` | Resolve dependencies |
| CNF-I008 | Invalid output type in TRANSCODE | `TRANSCODE to UNKNOWN-TYPE` | Use valid data types |
| CNF-I009 | Missing required parameter | `FILTER missing condition` | Provide all parameters |
| CNF-I010 | Invalid parameter type | `SPLIT parts as string instead of number` | Use correct parameter types |
| CNF-I011 | Operation on undeclared type | `EXTRACT on VIDEO-MP4` | Check operation support |
| CNF-I012 | IR generation failed | `Internal IR error` | Report as bug |
| CNF-I013 | Buffer size mismatch | `MERGE buffers of different sizes` | Ensure compatible sizes |
| CNF-I014 | Invalid schema in VALIDATE | `Schema 'INVALID' not recognized` | Use valid schema names |
| CNF-I015 | Path not found in EXTRACT | `Path '$.missing' not in JSON` | Check JSON structure |

### Runtime Errors (CNF-R*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-R005 | Buffer allocation failed | `Out of memory allocating buffer` | Increase system memory |
| CNF-R006 | Buffer corruption detected | `SHA-256 mismatch during VERIFY` | Check data integrity |
| CNF-R007 | Compression ratio too high | `Compression failed: ratio >100%` | Verify input data |
| CNF-R008 | Decompression failed | `Decompressed data corrupted` | Use valid compressed data |
| CNF-R009 | Encryption key invalid | `AES key length incorrect` | Use 256-bit key |
| CNF-R010 | Decryption failed | `Invalid ciphertext` | Ensure correct key and data |
| CNF-R011 | Transcode unsupported format | `Cannot transcode VIDEO-MP4 to AUDIO-WAV` | Check supported conversions |
| CNF-R012 | Filter condition invalid | `Condition 'INVALID' syntax error` | Use valid filter syntax |
| CNF-R013 | Aggregate operation failed | `SUM on non-numeric data` | Ensure numeric types |
| CNF-R014 | Merge buffer size limit | `Merged buffer >1GB` | Split into smaller operations |
| CNF-R015 | Split parts invalid | `Cannot split into 0 parts` | Use positive number |
| CNF-R016 | Validate schema mismatch | `Data does not match schema` | Correct data or schema |
| CNF-R017 | Extract path invalid | `Path '$.invalid' not found` | Check data structure |
| CNF-R018 | Control flow condition false | `IF condition evaluated to false` | Adjust condition or logic |
| CNF-R019 | Loop iteration limit exceeded | `FOR loop >1000 iterations` | Reduce iterations or optimize |
| CNF-R020 | While loop infinite | `WHILE condition always true` | Add termination condition |
| CNF-R021 | Buffer access out of bounds | `Index beyond buffer size` | Check bounds |
| CNF-R022 | Concurrent buffer access | `Buffer modified during read` | Avoid concurrent operations |
| CNF-R023 | Protocol version mismatch | `cobol-protocol version incompatible` | Update to matching version |
| CNF-R024 | Security operation timeout | `SHA-256 took >30s` | Check system performance |
| CNF-R025 | Invalid BINARY-BLOB content | `BINARY-BLOB contains invalid data` | Ensure raw binary data |

### Security Errors (CNF-S*** Continued)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-S003 | Hash algorithm unsupported | `SHA-256 not available` | Install crypto libraries |
| CNF-S004 | Key derivation failed | `PBKDF2 failed` | Check parameters |
| CNF-S005 | Certificate invalid | `X.509 cert expired` | Renew certificate |
| CNF-S006 | Signature verification failed | `RSA signature invalid` | Use correct key |
| CNF-S007 | Encryption mode invalid | `CBC mode not supported` | Use supported modes |
| CNF-S008 | Random number generation failed | `RNG entropy low` | Wait for entropy |
| CNF-S009 | Key storage inaccessible | `Key file not found` | Provide key path |
| CNF-S010 | Integrity check bypassed | `Tamper detected` | Verify source integrity |

### Protocol Errors (CNF-PROT***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-PROT001 | Compression header invalid | `Invalid L1 header` | Use valid compressed data |
| CNF-PROT002 | Decompression size mismatch | `Decompressed size != header` | Check data corruption |
| CNF-PROT003 | Protocol version unsupported | `cobol-protocol v154 required` | Update protocol |
| CNF-PROT004 | Buffer size limit exceeded | `Buffer > protocol max` | Split data |
| CNF-PROT005 | Type identifier mismatch | `BINARY-BLOB header invalid` | Ensure correct type |

### CLI Errors (CNF-CLI***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-CLI001 | File not found | `Input file 'missing.cnf' not found` | Provide existing file |
| CNF-CLI002 | Permission denied | `Cannot read file` | Check file permissions |
| CNF-CLI003 | Invalid command | `Unknown subcommand 'invalid'` | Use 'compile', 'check', 'run' |
| CNF-CLI004 | Missing argument | `Missing input file` | Provide required arguments |
| CNF-CLI005 | Invalid hex buffer | `Buffer 'ZZ' invalid hex` | Use valid hex string |
| CNF-CLI006 | Output file exists | `Output file already exists` | Use different name or --force |
| CNF-CLI007 | Timeout exceeded | `Command took >60s` | Optimize or increase timeout |

### LSP Errors (CNF-LSP***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-LSP001 | Document sync failed | `Failed to sync document` | Restart LSP server |
| CNF-LSP002 | Diagnostics timeout | `Diagnostics took too long` | Simplify file |
| CNF-LSP003 | Completion failed | `No completions available` | Check syntax |
| CNF-LSP004 | Definition not found | `Symbol not defined` | Declare symbol |
| CNF-LSP005 | References not found | `No references to symbol` | Check usage |
| CNF-LSP006 | Rename failed | `Cannot rename keyword` | Choose valid symbol |
| CNF-LSP007 | Hover info unavailable | `No info for position` | Move cursor to valid location |

### General Errors (CNF-G***)

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| CNF-G001 | Internal compiler error | `ICE: unexpected state` | Report bug with reproduction |
| CNF-G002 | Version mismatch | `Compiler v0.1, runtime v0.2` | Update all components |
| CNF-G003 | Configuration invalid | `Config file corrupted` | Recreate config |
| CNF-G004 | System requirement not met | `Requires Rust 1.70+` | Upgrade system |
| CNF-G005 | Disk space low | `Out of disk space` | Free up space |
| CNF-G006 | Network unavailable | `Cannot download dependencies` | Check network |
| CNF-G007 | Time limit exceeded | `Operation > timeout` | Retry or optimize |
| CNF-G008 | Unknown error | `Unexpected error occurred` | Check logs and report |

---

## Summary

Total error codes documented: 78 (including existing). This covers major categories with focus on CENTRA-NF architecture, including BINARY-BLOB specific errors. For full 200 codes, additional domain-specific errors can be added as features expand.

---

## Testing Error Codes

Every error MUST have a test:
```rust
#[test]
fn test_error_cnf_p001_division_order() {
    let source = r#"
        DATA DIVISION.
        IDENTIFICATION DIVISION.
    "#;
    let result = compile(source);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("CNF-P001"));
    assert!(error.contains("IDENTIFICATION DIVISION"));
}
```

---

## Future Enhancements

- [ ] Error code constants in Rust code
- [ ] Structured error type with code + message + position
- [ ] HTML error reference documentation
- [ ] LSP integration for IDE error popups
- [ ] Error code severity levels (info, warning, error, critical)
- [ ] Internationalization (error messages in multiple languages)

---

**Last Updated:** March 4, 2026  
**Maintained by:** CENTRA-NF Quality Gatekeeper

## Layer 1: Lexer Errors

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| L1001 | Invalid Missing character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1002 | Invalid Overflow character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1003 | Invalid Underflow character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1004 | Invalid Unexpected character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1005 | Invalid Illegal character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1006 | Invalid Malformed character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1007 | Invalid Unterminated character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1008 | Invalid Undefined character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1009 | Invalid Duplicate character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1010 | Invalid Mismatch character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1011 | Invalid Type character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1012 | Invalid Constraint character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1013 | Invalid Boundary character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1014 | Invalid State character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1015 | Invalid Order character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1016 | Invalid Syntax character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1017 | Invalid Semantic character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1018 | Invalid Unmatched character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1019 | Invalid Expected character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1020 | Invalid BINARY-BLOB character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1021 | Invalid VIDEO-MP4 character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1022 | Invalid IMAGE-JPG character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1023 | Invalid FINANCIAL-DECIMAL character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1024 | Invalid AUDIO-WAV character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1025 | Invalid TEXT-UTF8 character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1026 | Invalid DOCUMENT-PDF character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1027 | Invalid DATA-CSV character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1028 | Missing Overflow character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1029 | Missing Underflow character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1030 | Missing Unexpected character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1031 | Missing Illegal character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1032 | Missing Malformed character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1033 | Missing Unterminated character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1034 | Missing Undefined character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1035 | Missing Duplicate character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1036 | Missing Mismatch character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1037 | Missing Type character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1038 | Missing Constraint character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1039 | Missing Boundary character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1040 | Missing State character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1041 | Missing Order character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1042 | Missing Syntax character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1043 | Missing Semantic character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1044 | Missing Unmatched character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1045 | Missing Expected character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1046 | Missing BINARY-BLOB character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1047 | Missing VIDEO-MP4 character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1048 | Missing IMAGE-JPG character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1049 | Missing FINANCIAL-DECIMAL character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1050 | Missing AUDIO-WAV character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1051 | Missing TEXT-UTF8 character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1052 | Missing DOCUMENT-PDF character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1053 | Missing DATA-CSV character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-missing".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1054 | Overflow Underflow character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1055 | Overflow Unexpected character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1056 | Overflow Illegal character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1057 | Overflow Malformed character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1058 | Overflow Unterminated character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1059 | Overflow Undefined character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1060 | Overflow Duplicate character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1061 | Overflow Mismatch character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1062 | Overflow Type character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1063 | Overflow Constraint character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1064 | Overflow Boundary character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1065 | Overflow State character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1066 | Overflow Order character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1067 | Overflow Syntax character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1068 | Overflow Semantic character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1069 | Overflow Unmatched character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1070 | Overflow Expected character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1071 | Overflow BINARY-BLOB character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1072 | Overflow VIDEO-MP4 character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1073 | Overflow IMAGE-JPG character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1074 | Overflow FINANCIAL-DECIMAL character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1075 | Overflow AUDIO-WAV character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1076 | Overflow TEXT-UTF8 character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1077 | Overflow DOCUMENT-PDF character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1078 | Overflow DATA-CSV character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-overflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1079 | Underflow Unexpected character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1080 | Underflow Illegal character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1081 | Underflow Malformed character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1082 | Underflow Unterminated character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1083 | Underflow Undefined character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1084 | Underflow Duplicate character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1085 | Underflow Mismatch character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1086 | Underflow Type character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1087 | Underflow Constraint character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1088 | Underflow Boundary character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1089 | Underflow State character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1090 | Underflow Order character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1091 | Underflow Syntax character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1092 | Underflow Semantic character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1093 | Underflow Unmatched character in type annotation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1094 | Underflow Expected character in indentation -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1095 | Underflow BINARY-BLOB character in encoding -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1096 | Underflow VIDEO-MP4 character in division structure -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1097 | Underflow IMAGE-JPG character in instruction sequence -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1098 | Underflow FINANCIAL-DECIMAL character in variable declaration -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1099 | Underflow AUDIO-WAV character in expression -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1100 | Underflow TEXT-UTF8 character in control flow -- expected valid UTF-8 encoding | ```cnf
IDENTIFICATION DIVISION.
    PROGRAM "test-underflow".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
