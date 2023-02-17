## JVM Versioning Support

This library is intended to be used as a interface for reading and writing class files. As such, both the `Serializer` and `Deserialzier` will support the latest JVM version available. 
The onus of ensuring that the feature-set matches the correct JVM version as well as providing versioning metadata is on higher-level client (such aas `phoron_asm`).

## Version Support History

  - [JVMS 19](https://docs.oracle.com/javase/specs/jvms/se19/html/index.html) [✓]
