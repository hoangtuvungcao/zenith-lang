//! Zenith Platform Abstraction Layer
//! Cross-platform support for Zenith runtime and tools

use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::process::Command;

/// Platform detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    DragonFly,
    Android,
    IOS,
    Wasm,
    Unknown,
}

/// Architecture detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Architecture {
    X86,
    X86_64,
    Arm,
    Arm64,
    RiscV64,
    Mips,
    Mips64,
    PowerPc,
    PowerPc64,
    Sparc,
    Sparc64,
    Unknown,
}

/// Platform information
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub platform: Platform,
    pub architecture: Architecture,
    pub endian: Endian,
    pub pointer_width: usize,
    pub os_version: String,
    pub is_64bit: bool,
}

/// Endian detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

/// File system information
#[derive(Debug, Clone)]
pub struct FileSystemInfo {
    pub path_separator: String,
    pub line_ending: String,
    pub executable_extension: String,
    pub library_extension: String,
    pub case_sensitive: bool,
    pub supports_symlinks: bool,
}

/// System capabilities
#[derive(Debug, Clone)]
pub struct SystemCapabilities {
    pub supports_threads: bool,
    pub supports_fork: bool,
    pub supports_signals: bool,
    pub supports_mmap: bool,
    pub supports_shared_memory: bool,
    pub supports_pipes: bool,
    pub supports_sockets: bool,
    pub max_path_length: usize,
    pub max_filename_length: usize,
}

impl PlatformInfo {
    /// Get current platform information
    pub fn current() -> Self {
        let platform = detect_platform();
        let architecture = detect_architecture();
        let endian = detect_endian();
        let pointer_width = detect_pointer_width();
        let os_version = detect_os_version();
        let is_64bit = architecture.is_64bit();
        
        Self {
            platform,
            architecture,
            endian,
            pointer_width,
            os_version,
            is_64bit,
        }
    }
    
    /// Check if running on Windows
    pub fn is_windows(&self) -> bool {
        self.platform == Platform::Windows
    }
    
    /// Check if running on Unix-like system
    pub fn is_unix(&self) -> bool {
        matches!(self.platform, 
            Platform::Linux | Platform::MacOS | Platform::FreeBSD | 
            Platform::OpenBSD | Platform::NetBSD | Platform::DragonFly
        )
    }
    
    /// Check if running on mobile platform
    pub fn is_mobile(&self) -> bool {
        matches!(self.platform, Platform::Android | Platform::IOS)
    }
    
    /// Check if running on WebAssembly
    pub fn is_wasm(&self) -> bool {
        self.platform == Platform::Wasm
    }
    
    /// Get platform-specific executable name
    pub fn executable_name(&self, name: &str) -> String {
        if self.is_windows() {
            format!("{}.exe", name)
        } else {
            name.to_string()
        }
    }
    
    /// Get platform-specific library name
    pub fn library_name(&self, name: &str) -> String {
        if self.is_windows() {
            format!("{}.dll", name)
        } else if self.is_macos() {
            format!("lib{}.dylib", name)
        } else {
            format!("lib{}.so", name)
        }
    }
    
    /// Check if macOS
    pub fn is_macos(&self) -> bool {
        self.platform == Platform::MacOS
    }
    
    /// Check if Linux
    pub fn is_linux(&self) -> bool {
        self.platform == Platform::Linux
    }
}

impl FileSystemInfo {
    /// Get current file system information
    pub fn current() -> Self {
        let platform = PlatformInfo::current();
        
        match platform.platform {
            Platform::Windows => Self {
                path_separator: "\\".to_string(),
                line_ending: "\r\n".to_string(),
                executable_extension: "exe".to_string(),
                library_extension: "dll".to_string(),
                case_sensitive: false,
                supports_symlinks: false,
            },
            Platform::MacOS => Self {
                path_separator: "/".to_string(),
                line_ending: "\n".to_string(),
                executable_extension: "".to_string(),
                library_extension: "dylib".to_string(),
                case_sensitive: true,
                supports_symlinks: true,
            },
            _ => Self {
                path_separator: "/".to_string(),
                line_ending: "\n".to_string(),
                executable_extension: "".to_string(),
                library_extension: "so".to_string(),
                case_sensitive: true,
                supports_symlinks: true,
            },
        }
    }
    
    /// Join path components using platform separator
    pub fn join(&self, components: &[&str]) -> PathBuf {
        let mut path = PathBuf::new();
        for component in components {
            path.push(component);
        }
        path
    }
    
    /// Get home directory
    pub fn home_dir(&self) -> Option<PathBuf> {
        env::var_os("HOME")
            .and_then(|h| if h.len() > 0 { Some(PathBuf::from(h)) } else { None })
            .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
    }
    
    /// Get temp directory
    pub fn temp_dir(&self) -> PathBuf {
        env::temp_dir()
    }
    
    /// Get config directory
    pub fn config_dir(&self) -> PathBuf {
        let platform = PlatformInfo::current();
        
        if platform.is_windows() {
            env::var("APPDATA")
                .map(|s| PathBuf::from(s).join("Zenith"))
                .unwrap_or_else(|| {
                    let home = self.home_dir().unwrap_or_else(|| PathBuf::from("C:\\"));
                    home.join("AppData").join("Zenith")
                })
        } else if platform.is_macos() {
            self.home_dir()
                .unwrap_or_else(|| PathBuf::from("/Users/"))
                .join("Library")
                .join("Application Support")
                .join("Zenith")
        } else {
            // Linux and other Unix-like systems
            env::var("XDG_CONFIG_HOME")
                .map(|s| PathBuf::from(s).join("zenith"))
                .unwrap_or_else(|| {
                    self.home_dir()
                        .unwrap_or_else(|| PathBuf::from("/home/user"))
                        .join(".config")
                        .join("zenith")
                })
        }
    }
    
    /// Get cache directory
    pub fn cache_dir(&self) -> PathBuf {
        let platform = PlatformInfo::current();
        
        if platform.is_windows() {
            env::var("LOCALAPPDATA")
                .map(|s| PathBuf::from(s).join("Zenith").join("Cache"))
                .unwrap_or_else(|| {
                    let home = self.home_dir().unwrap_or_else(|| PathBuf::from("C:\\"));
                    home.join("AppData").join("Local").join("Zenith").join("Cache")
                })
        } else if platform.is_macos() {
            self.home_dir()
                .unwrap_or_else(|| PathBuf::from("/Users/"))
                .join("Library")
                .join("Caches")
                .join("Zenith")
        } else {
            // Linux and other Unix-like systems
            env::var("XDG_CACHE_HOME")
                .map(|s| PathBuf::from(s).join("zenith"))
                .unwrap_or_else(|| {
                    self.home_dir()
                        .unwrap_or_else(|| PathBuf::from("/home/user"))
                        .join(".cache")
                        .join("zenith")
                })
        }
    }
}

impl SystemCapabilities {
    /// Get current system capabilities
    pub fn current() -> Self {
        let platform = PlatformInfo::current();
        
        Self {
            supports_threads: true, // Most modern systems support threads
            supports_fork: platform.is_unix(),
            supports_signals: platform.is_unix(),
            supports_mmap: !platform.is_wasm(),
            supports_shared_memory: !platform.is_wasm(),
            supports_pipes: true,
            supports_sockets: !platform.is_wasm(),
            max_path_length: get_max_path_length(),
            max_filename_length: get_max_filename_length(),
        }
    }
    
    /// Check if system supports threading
    pub fn supports_threading(&self) -> bool {
        self.supports_threads
    }
    
    /// Check if system supports forking
    pub fn supports_forking(&self) -> bool {
        self.supports_fork
    }
    
    /// Check if system supports memory mapping
    pub fn supports_memory_mapping(&self) -> bool {
        self.supports_mmap
    }
}

impl Architecture {
    /// Check if architecture is 64-bit
    pub fn is_64bit(&self) -> bool {
        matches!(self, 
            Architecture::X86_64 | Architecture::Arm64 | 
            Architecture::RiscV64 | Architecture::Mips64 |
            Architecture::PowerPc64 | Architecture::Sparc64
        )
    }
    
    /// Check if architecture is little endian
    pub fn is_little_endian(&self) -> bool {
        detect_endian() == Endian::Little
    }
    
    /// Get architecture name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Architecture::X86 => "x86",
            Architecture::X86_64 => "x86_64",
            Architecture::Arm => "arm",
            Architecture::Arm64 => "arm64",
            Architecture::RiscV64 => "riscv64",
            Architecture::Mips => "mips",
            Architecture::Mips64 => "mips64",
            Architecture::PowerPc => "powerpc",
            Architecture::PowerPc64 => "powerpc64",
            Architecture::Sparc => "sparc",
            Architecture::Sparc64 => "sparc64",
            Architecture::Unknown => "unknown",
        }
    }
}

/// Detect current platform
fn detect_platform() -> Platform {
    #[cfg(target_os = "windows")]
    return Platform::Windows;
    
    #[cfg(target_os = "macos")]
    return Platform::MacOS;
    
    #[cfg(target_os = "linux")]
    return Platform::Linux;
    
    #[cfg(target_os = "freebsd")]
    return Platform::FreeBSD;
    
    #[cfg(target_os = "openbsd")]
    return Platform::OpenBSD;
    
    #[cfg(target_os = "netbsd")]
    return Platform::NetBSD;
    
    #[cfg(target_os = "dragonfly")]
    return Platform::DragonFly;
    
    #[cfg(target_os = "android")]
    return Platform::Android;
    
    #[cfg(target_os = "ios")]
    return Platform::IOS;
    
    #[cfg(target_arch = "wasm32")]
    return Platform::Wasm;
    
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "android",
        target_os = "ios",
        target_arch = "wasm32"
    )))]
    return Platform::Unknown;
}

/// Detect current architecture
fn detect_architecture() -> Architecture {
    #[cfg(target_arch = "x86")]
    return Architecture::X86;
    
    #[cfg(target_arch = "x86_64")]
    return Architecture::X86_64;
    
    #[cfg(target_arch = "arm")]
    return Architecture::Arm;
    
    #[cfg(target_arch = "aarch64")]
    return Architecture::Arm64;
    
    #[cfg(target_arch = "riscv64")]
    return Architecture::RiscV64;
    
    #[cfg(target_arch = "mips")]
    return Architecture::Mips;
    
    #[cfg(target_arch = "mips64")]
    return Architecture::Mips64;
    
    #[cfg(target_arch = "powerpc")]
    return Architecture::PowerPc;
    
    #[cfg(target_arch = "powerpc64")]
    return Architecture::PowerPc64;
    
    #[cfg(target_arch = "sparc")]
    return Architecture::Sparc;
    
    #[cfg(target_arch = "sparc64")]
    return Architecture::Sparc64;
    
    return Architecture::Unknown;
}

/// Detect endianness
fn detect_endian() -> Endian {
    const NUM: u32 = 0x01234567;
    const BYTES: [u8; 4] = unsafe { std::mem::transmute(NUM) };
    
    if BYTES[0] == 0x01 {
        Endian::Big
    } else {
        Endian::Little
    }
}

/// Detect pointer width
fn detect_pointer_width() -> usize {
    std::mem::size_of::<*const ()>()
}

/// Detect OS version
fn detect_os_version() -> String {
    #[cfg(target_os = "windows")]
    {
        if let Ok(version) = Command::new("cmd")
            .args(&["/c", "ver"])
            .output()
        {
            String::from_utf8_lossy(&version.stdout).trim().to_string()
        } else {
            "Unknown Windows".to_string()
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Ok(version) = Command::new("sw_vers")
            .args(&["-productVersion"])
            .output()
        {
            format!("macOS {}", String::from_utf8_lossy(&version.stdout).trim())
        } else {
            "Unknown macOS".to_string()
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(version) = fs::read_to_string("/etc/os-release") {
            version.lines()
                .find(|line| line.starts_with("PRETTY_NAME="))
                .and_then(|line| line.split('=').nth(1))
                .unwrap_or("Unknown Linux")
                .trim_matches('"')
                .to_string()
        } else {
            "Unknown Linux".to_string()
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "Unknown".to_string()
    }
}

/// Get maximum path length
fn get_max_path_length() -> usize {
    #[cfg(target_os = "windows")]
    260
    
    #[cfg(target_os = "macos")]
    1024
    
    #[cfg(target_os = "linux")]
    4096
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    1024
}

/// Get maximum filename length
fn get_max_filename_length() -> usize {
    #[cfg(target_os = "windows")]
    255
    
    #[cfg(target_os = "macos")]
    255
    
    #[cfg(target_os = "linux")]
    255
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    255
}

/// Platform-specific utilities
pub mod utils {
    use super::*;
    
    /// Get platform-specific environment variable
    pub fn get_env_var(key: &str) -> Option<String> {
        env::var(key).ok()
    }
    
    /// Set platform-specific environment variable
    pub fn set_env_var(key: &str, value: &str) {
        env::set_var(key, value);
    }
    
    /// Get platform-specific path separator
    pub fn path_separator() -> &'static str {
        if cfg!(target_os = "windows") {
            "\\"
        } else {
            "/"
        }
    }
    
    /// Get platform-specific line ending
    pub fn line_ending() -> &'static str {
        if cfg!(target_os = "windows") {
            "\r\n"
        } else {
            "\n"
        }
    }
    
    /// Check if file is executable on current platform
    pub fn is_executable(path: &Path) -> bool {
        #[cfg(target_os = "windows")]
        {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("exe"))
                .unwrap_or(false)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            fs::metadata(path)
                .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        }
    }
    
    /// Make file executable on current platform
    pub fn make_executable(path: &Path) -> Result<(), std::io::Error> {
        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms)?;
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows doesn't need special permissions for executables
        }
        
        Ok(())
    }
    
    /// Open file with default application
    pub fn open_with_default_app(path: &Path) -> Result<(), std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/c", &format!("start {}", path.display())])
                .spawn()?;
        }
        
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(path)
                .spawn()?;
        }
        
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(path)
                .spawn()?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = PlatformInfo::current();
        assert!(platform.platform != Platform::Unknown);
    }

    #[test]
    fn test_architecture_detection() {
        let arch = detect_architecture();
        assert!(arch != Architecture::Unknown);
    }

    #[test]
    fn test_endian_detection() {
        let endian = detect_endian();
        // Should be either Little or Big
        assert!(endian == Endian::Little || endian == Endian::Big);
    }

    #[test]
    fn test_filesystem_info() {
        let fs_info = FileSystemInfo::current();
        assert!(!fs_info.path_separator.is_empty());
    }

    #[test]
    fn test_system_capabilities() {
        let caps = SystemCapabilities::current();
        assert!(caps.supports_threads);
    }
}
