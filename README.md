# pmx_parser

**load '.pmx' file**
```
pub fn pmx_read<R: Read>(read: &mut R) -> Result<(Header, Pmx), PmxError>
```

**save '.pmx' file**
````
pub fn pmx_write<W: Write>(write: &mut W, pmx: &Pmx, version: f32) -> Result<(), PmxError>
```
