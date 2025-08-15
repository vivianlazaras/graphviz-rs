warning: wrappedviz@0.1.0: GRAPHVIZ_DIR not set, skipping link search path for Graphviz
   Compiling wrappedviz v0.1.0 (/home/cardinal/projects/maverics_respite/storyteller/graphviz)
warning: unused attribute `allow`
 --> src/sys.rs:1:1
  |
1 | #[allow(clippy::useless_transmute, clippy::transmute_ptr_to_ptr, clippy::transmute_ptr_to_ref)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the built-in attribute `allow` will be ignored, since it's applied to the macro invocation `include`
 --> src/sys.rs:2:1
  |
2 | include!("bindings.rs");
  | ^^^^^^^
  = note: `#[warn(unused_attributes)]` on by default

warning: unexpected `cfg` condition name: `dest_arch`
 --> src/rgraph/mod.rs:5:7
  |
5 | #[cfg(dest_arch = "wasm32")]
  |       ^^^^^^^^^^^^^^^^^^^^
  |
  = help: expected names are: `docsrs`, `feature`, and `test` and 31 more
  = help: consider using a Cargo feature instead
  = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
           [lints.rust]
           unexpected_cfgs = { level = "warn", check-cfg = ['cfg(dest_arch, values("wasm32"))'] }
  = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(dest_arch, values(\"wasm32\"))");` to the top of the `build.rs`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition name: `dest_arch`
 --> src/rgraph/mod.rs:7:7
  |
7 | #[cfg(dest_arch = "wasm32")]
  |       ^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider using a Cargo feature instead
  = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
           [lints.rust]
           unexpected_cfgs = { level = "warn", check-cfg = ['cfg(dest_arch, values("wasm32"))'] }
  = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(dest_arch, values(\"wasm32\"))");` to the top of the `build.rs`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

error[E0308]: mismatched types
    --> src/cgraph.rs:478:91
     |
478  |             if gvRenderData(self.ctx, graph.graph, format_cstr.as_ptr(), &mut result_ptr, &mut length as *mut u32) != 0 {
     |                ------------ arguments to this function are incorrect                      ^^^^^^^^^^^^^^^^^^^^^^^ expected `*mut usize`, found `*mut u32`
     |
     = note: expected raw pointer `*mut usize`
                found raw pointer `*mut u32`
note: function defined here
    --> src/bindings.rs:8324:12
     |
8324 |     pub fn gvRenderData(
     |            ^^^^^^^^^^^^
...
8329 |         length: *mut usize,
     |         ------

warning: unnecessary transmute
    --> src/bindings.rs:6888:18
     |
6888 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(0usize, 1u8) as u8 == 1)`
     |
     = note: `#[warn(unnecessary_transmutes)]` on by default

warning: unnecessary transmute
    --> src/bindings.rs:6893:27
     |
6893 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6900:13
     |
6900 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
6901 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
6902 | |                 0usize,
6903 | |                 1u8,
6904 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
6900 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
6901 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
6902 +                 0usize,
6903 +                 1u8,
6904 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:6910:27
     |
6910 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6921:18
     |
6921 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(1usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:6926:27
     |
6926 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6933:13
     |
6933 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
6934 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
6935 | |                 1usize,
6936 | |                 1u8,
6937 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
6933 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
6934 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
6935 +                 1usize,
6936 +                 1u8,
6937 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:6943:27
     |
6943 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6954:18
     |
6954 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(2usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:6959:27
     |
6959 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6966:13
     |
6966 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
6967 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
6968 | |                 2usize,
6969 | |                 1u8,
6970 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
6966 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
6967 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
6968 +                 2usize,
6969 +                 1u8,
6970 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:6976:27
     |
6976 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6987:18
     |
6987 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(3usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:6992:27
     |
6992 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:6999:13
     |
6999 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7000 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7001 | |                 3usize,
7002 | |                 1u8,
7003 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
6999 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7000 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7001 +                 3usize,
7002 +                 1u8,
7003 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7009:27
     |
7009 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7020:18
     |
7020 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(4usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7025:27
     |
7025 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7032:13
     |
7032 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7033 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7034 | |                 4usize,
7035 | |                 1u8,
7036 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7032 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7033 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7034 +                 4usize,
7035 +                 1u8,
7036 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7042:27
     |
7042 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7053:18
     |
7053 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(5usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7058:27
     |
7058 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7065:13
     |
7065 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7066 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7067 | |                 5usize,
7068 | |                 1u8,
7069 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7065 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7066 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7067 +                 5usize,
7068 +                 1u8,
7069 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7075:27
     |
7075 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7086:18
     |
7086 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(6usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7091:27
     |
7091 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7098:13
     |
7098 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7099 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7100 | |                 6usize,
7101 | |                 1u8,
7102 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7098 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7099 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7100 +                 6usize,
7101 +                 1u8,
7102 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7108:27
     |
7108 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7119:18
     |
7119 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(7usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7124:27
     |
7124 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7131:13
     |
7131 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7132 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7133 | |                 7usize,
7134 | |                 1u8,
7135 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7131 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7132 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7133 +                 7usize,
7134 +                 1u8,
7135 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7141:27
     |
7141 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7152:18
     |
7152 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(8usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7157:27
     |
7157 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7164:13
     |
7164 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7165 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7166 | |                 8usize,
7167 | |                 1u8,
7168 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7164 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7165 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7166 +                 8usize,
7167 +                 1u8,
7168 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7174:27
     |
7174 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7185:18
     |
7185 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(9usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7190:27
     |
7190 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7197:13
     |
7197 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7198 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7199 | |                 9usize,
7200 | |                 1u8,
7201 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7197 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7198 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7199 +                 9usize,
7200 +                 1u8,
7201 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7207:27
     |
7207 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7218:18
     |
7218 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(10usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7223:27
     |
7223 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7230:13
     |
7230 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7231 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7232 | |                 10usize,
7233 | |                 1u8,
7234 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7230 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7231 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7232 +                 10usize,
7233 +                 1u8,
7234 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7240:27
     |
7240 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7251:18
     |
7251 |         unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u8) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(self._bitfield_1.get(11usize, 1u8) as u8 == 1)`

warning: unnecessary transmute
    --> src/bindings.rs:7256:27
     |
7256 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7263:13
     |
7263 | /             ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7264 | |                 ::std::ptr::addr_of!((*this)._bitfield_1),
7265 | |                 11usize,
7266 | |                 1u8,
7267 | |             ) as u8)
     | |____________________^
     |
help: replace this with
     |
7263 ~             (<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(
7264 +                 ::std::ptr::addr_of!((*this)._bitfield_1),
7265 +                 11usize,
7266 +                 1u8,
7267 +             ) as u8 == 1)
     |

warning: unnecessary transmute
    --> src/bindings.rs:7273:27
     |
7273 |             let val: u8 = ::std::mem::transmute(val);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(val) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7333:39
     |
7333 |             let filled: u8 = unsafe { ::std::mem::transmute(filled) };
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(filled) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7337:39
     |
7337 |             let radial: u8 = unsafe { ::std::mem::transmute(radial) };
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(radial) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7341:40
     |
7341 |             let rounded: u8 = unsafe { ::std::mem::transmute(rounded) };
     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(rounded) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7345:42
     |
7345 |             let diagonals: u8 = unsafe { ::std::mem::transmute(diagonals) };
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(diagonals) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7349:42
     |
7349 |             let auxlabels: u8 = unsafe { ::std::mem::transmute(auxlabels) };
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(auxlabels) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7353:42
     |
7353 |             let invisible: u8 = unsafe { ::std::mem::transmute(invisible) };
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(invisible) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7357:40
     |
7357 |             let striped: u8 = unsafe { ::std::mem::transmute(striped) };
     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(striped) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7361:39
     |
7361 |             let dotted: u8 = unsafe { ::std::mem::transmute(dotted) };
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(dotted) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7365:39
     |
7365 |             let dashed: u8 = unsafe { ::std::mem::transmute(dashed) };
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(dashed) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7369:39
     |
7369 |             let wedged: u8 = unsafe { ::std::mem::transmute(wedged) };
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(wedged) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7373:42
     |
7373 |             let underline: u8 = unsafe { ::std::mem::transmute(underline) };
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(underline) as u8`

warning: unnecessary transmute
    --> src/bindings.rs:7377:43
     |
7377 |             let fixedshape: u8 = unsafe { ::std::mem::transmute(fixedshape) };
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace this with: `(fixedshape) as u8`

For more information about this error, try `rustc --explain E0308`.
warning: `wrappedviz` (lib) generated 63 warnings
warning: wrappedviz@0.1.0: GRAPHVIZ_DIR not set, skipping link search path for Graphviz
error: could not compile `wrappedviz` (lib) due to 1 previous error; 63 warnings emitted
