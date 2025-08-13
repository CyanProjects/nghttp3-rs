extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lib_version = env::var("CARGO_PKG_VERSION")
        .unwrap()
        .split('+')
        .nth(1)
        .unwrap()
        .to_string();
    let major = lib_version
        .split('.')
        .nth(0)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let minor = lib_version
        .split('.')
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let patch = lib_version
        .split('.')
        .nth(2)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let ver = fs::read_to_string("nghttp3/lib/includes/nghttp3/version.h.in")
        .unwrap()
        .replace("@PACKAGE_VERSION@", &lib_version)
        .replace(
            "@PACKAGE_VERSION_NUM@",
            &format!("0x{:02x}{:02x}{:02x}", major, minor, patch),
        );

    let install = out_dir.join("i");

    let include = install.join("include");
    let lib = install.join("lib");
    let pkgconfig = lib.join("pkgconfig");
    fs::create_dir_all(include.join("nghttp3")).unwrap();
    fs::create_dir_all(&pkgconfig).unwrap();
    fs::write(include.join("nghttp3/version.h"), ver).unwrap();
    fs::copy(
        "nghttp3/lib/includes/nghttp3/nghttp3.h",
        include.join("nghttp3/nghttp3.h"),
    )
        .unwrap();

    let mut cfg = cc::Build::new();
    cfg.include("nghttp3/lib/includes")
        .include(&include).file("nghttp3/lib/nghttp3_balloc.c")
        .file("nghttp3/lib/sfparse/sfparse.c")
        .file("nghttp3/lib/nghttp3_buf.c")
        .file("nghttp3/lib/nghttp3_callbacks.c")
        .file("nghttp3/lib/nghttp3_conn.c")
        .file("nghttp3/lib/nghttp3_conv.c")
        .file("nghttp3/lib/nghttp3_debug.c")
        .file("nghttp3/lib/nghttp3_err.c")
        .file("nghttp3/lib/nghttp3_frame.c")
        .file("nghttp3/lib/nghttp3_gaptr.c")
        .file("nghttp3/lib/nghttp3_http.c")
        .file("nghttp3/lib/nghttp3_idtr.c")
        .file("nghttp3/lib/nghttp3_ksl.c")
        .file("nghttp3/lib/nghttp3_map.c")
        .file("nghttp3/lib/nghttp3_mem.c")
        .file("nghttp3/lib/nghttp3_objalloc.c")
        .file("nghttp3/lib/nghttp3_opl.c")
        .file("nghttp3/lib/nghttp3_pq.c")
        .file("nghttp3/lib/nghttp3_qpack.c")
        .file("nghttp3/lib/nghttp3_qpack_huffman.c")
        .file("nghttp3/lib/nghttp3_qpack_huffman_data.c")
        .file("nghttp3/lib/nghttp3_range.c")
        .file("nghttp3/lib/nghttp3_rcbuf.c")
        .file("nghttp3/lib/nghttp3_ringbuf.c")
        .file("nghttp3/lib/nghttp3_settings.c")
        .file("nghttp3/lib/nghttp3_str.c")
        .file("nghttp3/lib/nghttp3_stream.c")
        .file("nghttp3/lib/nghttp3_tnode.c")
        .file("nghttp3/lib/nghttp3_unreachable.c")
        .file("nghttp3/lib/nghttp3_vec.c")
        .file("nghttp3/lib/nghttp3_version.c")
        .warnings(false)
        .define("HAVE_UNISTD_H", None)
        .define("NGHTTP3_STATICLIB", None)
        .define("HAVE_NETINET_IN", None)
        .define("HAVE_TIME_H", None)
        .out_dir(&lib);

    if target.contains("windows") {
        // Apparently MSVC doesn't have `ssize_t` defined as a type
        if target.contains("msvc") {
            match &env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap()[..] {
                "64" => {
                    cfg.define("ssize_t", "int64_t");
                }
                "32" => {
                    cfg.define("ssize_t", "int32_t");
                }
                s => panic!("unknown pointer size: {}", s),
            }
        }
    } else {
        cfg.define("HAVE_ARPA_INET_H", None);
    }
    cfg.compile("nghttp3");

    println!("cargo:root={}", install.display());

    let pc = fs::read_to_string("nghttp3/lib/libnghttp3.pc.in")
        .unwrap()
        .replace("@prefix@", install.to_str().unwrap())
        .replace("@exec_prefix@", "")
        .replace("@libdir@", lib.to_str().unwrap())
        .replace("@includedir@", include.to_str().unwrap())
        .replace("@VERSION@", &lib_version);
    fs::write(pkgconfig.join("libnghttp3.pc"), pc).unwrap();
}
