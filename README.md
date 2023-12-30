## Overview
check if the given ports is available, get available ports

## Usage

add the dependent

```sh
cargo add port
```

```rust
use port::{check_port};

let is_port_available = check_port("127.0.0.1", 8080);
```