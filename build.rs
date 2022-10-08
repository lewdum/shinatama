const LANG: u16 = 1033; // English (United States)

fn main() -> std::io::Result<()> {
    let mut res = winres::WindowsResource::new();
    res.set_language(LANG)
        .set("FileDescription", "Miscellaneous engine patches for Oni.")
        .set("ProductName", "Shinatama Patches")
        .set("OriginalFilename", "dinput.dll")
        .set("LegalCopyright", "github.com/lewdum");
    res.compile()
}
