# Disk Stress Test

A low-level storage validation tool written in Rust that performs concurrent CPU and disk I/O stress testing. Designed for hardware validation scenarios where CPU thermal conditions or power delivery issues may impact storage reliability.

## ⚠️ Safety Notice

This tool performs intensive disk operations while maintaining high CPU load, which can trigger system instability. Always test on development systems with no critical data. Running stress tests can potentially accelerate hardware failure on systems with existing issues.

- Test on a separate drive from your OS with no valuable data
- Monitor system temperatures and storage controller warnings during testing

## Build Instructions

### 1. Prerequisites & Build
```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/yourusername/disk-stress-test
cd disk-stress-test
cargo build --release
```

### 2. System-specific Requirements

2a. Linux:
```bash
sudo apt install libc-dev linux-headers-$(uname -r)  # Debian/Ubuntu
```

2b. Windows:
- Install Visual Studio Build Tools with Windows SDK

## Usage

```bash
disk-stress-test --dir /path/to/test --chunk-size 4 --cpu-load 90
```

### Parameters

- `--dir`: Test directory path (will be created if needed)
- `--chunk-size`: Test file size in GB (default: 4)
- `--cpu-load`: Target CPU utilization percentage (default: 90)

## Technical Details

### Implementation Notes
- Uses Rust's async runtime for I/O operations
- Implements memory-mapped file I/O for direct disk access
- CPU stress testing via parallel prime number generation
- Custom pattern generation for corruption detection
- Real-time CRC32 verification of written data

### Performance Considerations
- Memory-mapped I/O may trigger kernel page cache
- CPU governor settings affect stress test consistency
- Storage controller buffering can mask issues
- NUMA systems may see varying results per node

### Known Limitations
- May not detect intermittent errors under 1ms
- Storage controller caching can affect results
- Not suitable for testing encrypted volumes
- Performance varies with filesystem choice

## License

MIT License - Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy of this software, to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.