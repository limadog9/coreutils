// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use divan::{Bencher, black_box};
use tempfile::TempDir;
use uu_dd::uumain;
use uucore::benchmark::{binary_data, fs_utils, run_util_function};

/// Benchmark basic dd copy with default settings
#[divan::bench]
fn dd_copy_default(bencher: Bencher) {
    let size_mb = 32;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd copy with 4KB block size (common page size)
#[divan::bench]
fn dd_copy_4k_blocks(bencher: Bencher) {
    let size_mb = 24;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=4K",
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd copy with 64KB block size
#[divan::bench]
fn dd_copy_64k_blocks(bencher: Bencher) {
    let size_mb = 64;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=64K",
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd copy with 1MB block size
#[divan::bench]
fn dd_copy_1m_blocks(bencher: Bencher) {
    let size_mb = 128;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=1M",
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd copy with separate input and output block sizes
#[divan::bench]
fn dd_copy_separate_blocks(bencher: Bencher) {
    let size_mb = 48;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "ibs=8K",
                "obs=16K",
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd with count limit (partial copy)
#[divan::bench]
fn dd_copy_partial(bencher: Bencher) {
    let size_mb = 32;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=4K",
                "count=1024",
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd with skip (seeking in input)
#[divan::bench]
fn dd_copy_with_skip(bencher: Bencher) {
    let size_mb = 48;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=4K",
                "skip=256",
                "status=none",
            ],
        ));
    });
}

/// Benchmark skipping a non-seekable stream instead of seeking a regular file.
#[cfg(target_os = "linux")]
#[divan::bench(args = [1, 32])]
fn dd_copy_with_stream_skip(bencher: Bencher, skip_mib: usize) {
    use std::io::{self, Write};
    use std::os::fd::AsRawFd;
    use std::thread;

    bencher
        .with_inputs(|| {
            let temp_dir = TempDir::new().unwrap();
            let (reader, mut input) = io::pipe().unwrap();
            let writer = thread::spawn(move || -> io::Result<()> {
                let buffer = [b'x'; 64 * 1024];
                for _ in 0..skip_mib * 16 {
                    input.write_all(&buffer)?;
                }
                input.write_all(&[b'y'; 4096])
            });
            (temp_dir, reader, writer)
        })
        .bench_local_values(|(temp_dir, reader, writer)| {
            let input = format!("if=/proc/self/fd/{}", reader.as_raw_fd());
            let output = temp_dir.path().join("output.bin");
            assert_eq!(
                black_box(run_util_function(
                    uumain,
                    &[
                        &input,
                        &format!("of={}", output.display()),
                        "bs=4K",
                        &format!("skip={}", skip_mib * 256),
                        "count=1",
                        "iflag=fullblock",
                        "status=none",
                    ],
                )),
                0
            );
            drop(reader);
            writer.join().unwrap().unwrap();
            assert_eq!(std::fs::read(output).unwrap(), [b'y'; 4096]);
        });
}

/// Benchmark dd with seek (seeking in output)
#[divan::bench]
fn dd_copy_with_seek(bencher: Bencher) {
    let size_mb = 48;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=4K",
                "seek=256",
                "status=none",
            ],
        ));
    });
}

/// Benchmark dd with different block sizes for comparison
#[divan::bench]
fn dd_copy_8k_blocks(bencher: Bencher) {
    let size_mb = 32;
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.bin");
    let output = temp_dir.path().join("output.bin");

    binary_data::create_file(&input, size_mb, b'x');

    let input_str = input.to_str().unwrap();
    let output_str = output.to_str().unwrap();

    bencher.bench(|| {
        fs_utils::remove_path(&output);
        black_box(run_util_function(
            uumain,
            &[
                &format!("if={input_str}"),
                &format!("of={output_str}"),
                "bs=8K",
                "status=none",
            ],
        ));
    });
}

fn main() {
    divan::main();
}
