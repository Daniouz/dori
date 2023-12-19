# Dori - Clint

Clint is the Dori client installer.

## How to Use

Create a folder on the client computer. It should look like this:

```
\
 - dori-client.exe
 - dori-clint.exe
 - .dori.toml
```

### Example .dori.toml

```toml
client_name = "test-client"
program_name = "DoriTestClient"

host_address = "127.0.0.1:12700"
key = "testpass"
```