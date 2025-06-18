#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(improper_ctypes)]

include!("bindings.rs");
use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Circo,
    Dot,
    Fdp,
    Neato,
    Nop,
    Nop1,
    Nop2,
    Osage,
    Patchwork,
    Sfdp,
    Twopi,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Layout::Circo => "circo",
            Layout::Dot => "dot",
            Layout::Fdp => "fdp",
            Layout::Neato => "neato",
            Layout::Nop => "nop",
            Layout::Nop1 => "nop1",
            Layout::Nop2 => "nop2",
            Layout::Osage => "osage",
            Layout::Patchwork => "patchwork",
            Layout::Sfdp => "sfdp",
            Layout::Twopi => "twopi",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Layout {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "circo" => Ok(Layout::Circo),
            "dot" => Ok(Layout::Dot),
            "fdp" => Ok(Layout::Fdp),
            "neato" => Ok(Layout::Neato),
            "nop" => Ok(Layout::Nop),
            "nop1" => Ok(Layout::Nop1),
            "nop2" => Ok(Layout::Nop2),
            "osage" => Ok(Layout::Osage),
            "patchwork" => Ok(Layout::Patchwork),
            "sfdp" => Ok(Layout::Sfdp),
            "twopi" => Ok(Layout::Twopi),
            other => Err(format!("Unknown layout variant '{}'", other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Canon,
    Cmap,
    Cmapx,
    CmapxNp,
    Dot,
    DotJson,
    Eps,
    Fig,
    Gd,
    Gd2,
    Gif,
    Gv,
    Imap,
    ImapNp,
    Ismap,
    Jpe,
    Jpeg,
    Jpg,
    Json,
    Json0,
    Kitty,
    Kittyz,
    Pdf,
    Pic,
    Plain,
    PlainExt,
    Png,
    Pov,
    Ps,
    Ps2,
    Svg,
    SvgInline,
    Svgz,
    Tk,
    Vrml,
    Vt,
    Vt24bit,
    Vt4up,
    Vt6up,
    Vt8up,
    Wbmp,
    Xdot,
    Xdot12,
    Xdot14,
    XdotJson,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OutputFormat::Canon => "canon",
            OutputFormat::Cmap => "cmap",
            OutputFormat::Cmapx => "cmapx",
            OutputFormat::CmapxNp => "cmapx_np",
            OutputFormat::Dot => "dot",
            OutputFormat::DotJson => "dot_json",
            OutputFormat::Eps => "eps",
            OutputFormat::Fig => "fig",
            OutputFormat::Gd => "gd",
            OutputFormat::Gd2 => "gd2",
            OutputFormat::Gif => "gif",
            OutputFormat::Gv => "gv",
            OutputFormat::Imap => "imap",
            OutputFormat::ImapNp => "imap_np",
            OutputFormat::Ismap => "ismap",
            OutputFormat::Jpe => "jpe",
            OutputFormat::Jpeg => "jpeg",
            OutputFormat::Jpg => "jpg",
            OutputFormat::Json => "json",
            OutputFormat::Json0 => "json0",
            OutputFormat::Kitty => "kitty",
            OutputFormat::Kittyz => "kittyz",
            OutputFormat::Pdf => "pdf",
            OutputFormat::Pic => "pic",
            OutputFormat::Plain => "plain",
            OutputFormat::PlainExt => "plain_ext",
            OutputFormat::Png => "png",
            OutputFormat::Pov => "pov",
            OutputFormat::Ps => "ps",
            OutputFormat::Ps2 => "ps2",
            OutputFormat::Svg => "svg",
            OutputFormat::SvgInline => "svg_inline",
            OutputFormat::Svgz => "svgz",
            OutputFormat::Tk => "tk",
            OutputFormat::Vrml => "vrml",
            OutputFormat::Vt => "vt",
            OutputFormat::Vt24bit => "vt-24bit",
            OutputFormat::Vt4up => "vt-4up",
            OutputFormat::Vt6up => "vt-6up",
            OutputFormat::Vt8up => "vt-8up",
            OutputFormat::Wbmp => "wbmp",
            OutputFormat::Xdot => "xdot",
            OutputFormat::Xdot12 => "xdot1.2",
            OutputFormat::Xdot14 => "xdot1.4",
            OutputFormat::XdotJson => "xdot_json",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "canon" => Ok(OutputFormat::Canon),
            "cmap" => Ok(OutputFormat::Cmap),
            "cmapx" => Ok(OutputFormat::Cmapx),
            "cmapx_np" | "cmapxnp" => Ok(OutputFormat::CmapxNp),
            "dot" => Ok(OutputFormat::Dot),
            "dot_json" | "dotjson" => Ok(OutputFormat::DotJson),
            "eps" => Ok(OutputFormat::Eps),
            "fig" => Ok(OutputFormat::Fig),
            "gd" => Ok(OutputFormat::Gd),
            "gd2" => Ok(OutputFormat::Gd2),
            "gif" => Ok(OutputFormat::Gif),
            "gv" => Ok(OutputFormat::Gv),
            "imap" => Ok(OutputFormat::Imap),
            "imap_np" | "imapnp" => Ok(OutputFormat::ImapNp),
            "ismap" => Ok(OutputFormat::Ismap),
            "jpe" => Ok(OutputFormat::Jpe),
            "jpeg" => Ok(OutputFormat::Jpeg),
            "jpg" => Ok(OutputFormat::Jpg),
            "json" => Ok(OutputFormat::Json),
            "json0" => Ok(OutputFormat::Json0),
            "kitty" => Ok(OutputFormat::Kitty),
            "kittyz" => Ok(OutputFormat::Kittyz),
            "pdf" => Ok(OutputFormat::Pdf),
            "pic" => Ok(OutputFormat::Pic),
            "plain" => Ok(OutputFormat::Plain),
            "plain_ext" | "plainext" => Ok(OutputFormat::PlainExt),
            "png" => Ok(OutputFormat::Png),
            "pov" => Ok(OutputFormat::Pov),
            "ps" => Ok(OutputFormat::Ps),
            "ps2" => Ok(OutputFormat::Ps2),
            "svg" => Ok(OutputFormat::Svg),
            "svg_inline" | "svginline" => Ok(OutputFormat::SvgInline),
            "svgz" => Ok(OutputFormat::Svgz),
            "tk" => Ok(OutputFormat::Tk),
            "vrml" => Ok(OutputFormat::Vrml),
            "vt" => Ok(OutputFormat::Vt),
            "vt-24bit" | "vt24bit" => Ok(OutputFormat::Vt24bit),
            "vt-4up" | "vt4up" => Ok(OutputFormat::Vt4up),
            "vt-6up" | "vt6up" => Ok(OutputFormat::Vt6up),
            "vt-8up" | "vt8up" => Ok(OutputFormat::Vt8up),
            "wbmp" => Ok(OutputFormat::Wbmp),
            "xdot" => Ok(OutputFormat::Xdot),
            "xdot1.2" | "xdot12" => Ok(OutputFormat::Xdot12),
            "xdot1.4" | "xdot14" => Ok(OutputFormat::Xdot14),
            "xdot_json" | "xdotjson" => Ok(OutputFormat::XdotJson),
            other => Err(format!("Unknown output format variant '{}'", other)),
        }
    }
}

pub struct Graph<'c> {
    graph: *mut Agraph_t,
    layout: Option<Layout>,
    ctx: &'c Context,
}

impl<'c> Graph<'c> {
    pub fn new<S: AsRef<str>>(dot: S, ctx: &'c Context) -> Self {
        let str_slice = dot.as_ref();

        // Convert Rust &str to CString
        let c_dot = CString::new(str_slice).expect("Input dot string contains interior null bytes");

        // Call the C function with a pointer to the CString buffer
        let graph = unsafe { agmemread(c_dot.as_ptr()) };

        Self {
            graph,
            layout: None,
            ctx
        }
        // Note: c_dot is dropped here, which is OK if agmemread copies the data,
        // else you'd need to keep c_dot alive somewhere.
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
        let layout_str = layout.to_string();

        let c_layout =
            CString::new(layout_str).expect("Layout string contains interior null bytes");

        // Call the layout function with the layout string pointer
        let result = unsafe { gvLayout(self.ctx.ctx, self.graph, c_layout.as_ptr()) };

        if result != 0 {
            panic!("gvLayout failed with error code {}", result);
        }
    }
}

impl<'c> std::ops::Drop for Graph<'c> {
    fn drop(&mut self) {
        unsafe{
            if let Some(_) = self.layout {
                gvFreeLayout(self.ctx.ctx, self.graph);
            }
            agclose(self.graph);
        }
    }
}

pub struct Context {
    ctx: *mut GVC_t,
}

impl Context {
    pub fn new() -> Context {
        Self {
            ctx: unsafe { gvContext() },
        }
    }

    pub fn render(&self, graph: Graph, format: OutputFormat) -> Vec<u8> {
        let format_str = format.to_string();
        let mut result_ptr: *mut ::std::os::raw::c_char = ptr::null_mut();
        let mut length: usize = 0;
        unsafe {
            if gvRenderData(
                self.ctx,
                graph.graph,
                CString::new(format_str.as_bytes().to_vec()).unwrap().as_ptr(),
                &mut result_ptr,
                &mut length,
            ) != 0
            {
                panic!("Render failed");
            }

            // Convert to Rust string
            let svg_slice = std::slice::from_raw_parts(result_ptr as *const u8, length as usize);
            let result = svg_slice.to_vec();
            gvFreeRenderData(result_ptr);
            result
        }
    }
}

impl std::ops::Drop for Context {
    fn drop(&mut self) {
        unsafe { gvFreeContext(self.ctx); }
    }
}