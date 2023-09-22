use std::fs;
use dirs::download_dir;
use std::path::Path;
use native_dialog::MessageType;
use native_dialog::MessageDialog;
use std::time::Duration;

//declare file extensions
const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "jpe", "jif", "jfif", "jfi", "png", "gif", "webp", "tiff",
    "tif", "psd", "raw", "arw", "cr2", "nrw", "k25", "bmp", "dib", "heif",
    "heic", "ind", "indd", "indt", "jp2", "j2k", "jpf", "jpf", "jpx", "jpm",
    "mj2", "svg", "svgz", "ai", "eps", "ico",
];

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "3gp", "ogv",
    "mpeg", "mpg", "ts", "asf", "rm", "swf", "vob", "divx", "m2v", "mpv",
    "m1v", "ogg", "ogm", "m2ts", "mts", "f4v", "rmvb", "amv", "drc", "m2p",
    "m2t", "mjp", "qt", "xvid", "yuv", "cinepak", "cavs", "dirac", "dv",
    "flic", "h264", "h265", "itp", "k3g", "libx264", "libx265", "m4p", "m4v",
];

const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "ogg", "flac", "m4a", "aac", "wma", "ape", "opus", "ac3",
    "amr", "mid", "midi", "kar", "rmi", "mod", "xm", "s3m", "it", "mtm",
    "umx", "mo3", "wavpack", "wv", "ape", "mpc", "tta", "dsf", "dff", "awb",
    "opus", "webm", "ra", "rm", "dts", "dtsma", "dtshd", "mp2", "msv", "vox",
    "aac", "adts", "alac", "aif", "aiff", "caf", "flp", "gsm", "mka", "mlp",
];

const DOCUMENT_EXTENSIONS: &[&str] = &[
    "pdf", "doc", "docx", "txt", "rtf", "odt", "ods", "csv", "xls", "xlsx",
    "ppt", "pptx", "odp", "html", "htm", "xml", "json", "md", "markdown",
    "tex", "log", "ini", "cfg", "yaml", "yml", "json", "rtf", "sxw", "stw",
    "odf", "ott", "ott", "stc", "sxc", "sxi", "sxd", "pot", "pps", "odb",
    "dbf", "accdb", "mdb", "sql", "sqlite", "sqlitedb",
];

const PROGRAM_EXTENSIONS: &[&str] = &[
    "exe", "msi", "app", "bat", "sh", "bash", "csh", "cmd", "com", "ps1", "vbs",
    "jar", "py", "pl", "rb", "cpp", "c", "h", "hpp", "cs", "java", "class",
    "dll", "so", "rpm", "deb", "apk", "swift", "go", "php", "asp", "aspx", "jsp",
    "cgi", "perl", "lua", "dart", "ts", "js", "html", "css", "scss", "less",
    "coffee", "tsx", "vb", "r", "scala", "rust", "kt", "kts", "hs", "lisp", "d",
];

const ARCHIVE_EXTENSIONS: &[&str] = &[
    "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "z", "cab", "iso", "vhd",
    "vmdk", "wim", "dmg", "tgz", "tbz2", "lzma", "rz", "deb", "rpm", "sit",
    "sitx", "sea", "arj", "lha", "ace", "gzip", "cab", "jar", "war", "ear",
    "apk", "zpaq", "pak", "arc", "lzh", "s7z", "ace", "r01", "r02", "r03",
    "r04", "r05", "r06", "r07", "r08", "r09", "r10", "r11", "r12", "r13",
    "r14", "r15", "r16", "r17", "r18", "r19", "r20", "r21", "r22", "r23",
    "r24", "r25", "r26", "r27", "r28", "r29", "r30", "r31", "r32", "r33",
    "r34", "r35", "r36", "r37", "r38", "r39", "r40", "r41", "r42", "r43",
    "r44", "r45", "r46", "r47", "r48", "r49", "r50", "r51", "r52", "r53",
    "r54", "r55", "r56", "r57", "r58", "r59", "r60", "r61", "r62", "r63",
    "r64", "r65", "r66", "r67", "r68", "r69", "r70", "r71", "r72", "r73",
    "r74", "r75", "r76", "r77", "r78", "r79", "r80", "r81", "r82", "r83",
    "r84", "r85", "r86", "r87", "r88", "r89", "r90", "r91", "r92", "r93",
    "r94", "r95", "r96", "r97", "r98", "r99", "r100",
];

const DIRECTORIES_TO_CHECK: [&str; 6] = [
    "Downloaded Images",
    "Downloaded Videos",
    "Downloaded Audios",
    "Downloaded Documents",
    "Downloaded Programs",
    "Downloaded Archives",
];

const DIRECTORIES_MAP: &[(&[&str], &str)] = &[
    (&IMAGE_EXTENSIONS, "Downloaded Images"),
    (&VIDEO_EXTENSIONS, "Downloaded Videos"),
    (&AUDIO_EXTENSIONS, "Downloaded Audios"),
    (&DOCUMENT_EXTENSIONS, "Downloaded Documents"),
    (&PROGRAM_EXTENSIONS, "Downloaded Programs"),
    (&ARCHIVE_EXTENSIONS, "Downloaded Archives"),
];

fn get_file_extension(file_name: &str) -> Option<&str> {
    let path = Path::new(file_name);
    if let Some(ext) = path.extension() {
        Some(ext.to_str().unwrap())
    } else {
        None
    }
}

fn check_directories_existence(download_dir: &Path) {
    //iterate through the directory names and check if each one exists
    for dir_name in &DIRECTORIES_TO_CHECK {
        let dest_dir = download_dir.join(dir_name);
        if !dest_dir.exists() {
            //create the directory if it doesn't exist
            fs::create_dir(dest_dir).expect("Unable to create directory");
        }
    }
}

fn find_target_directory(file_extension: &str) -> Option<&str> {
    // Convert to lowercase
    let lowercase_extension = file_extension.to_lowercase();
    for (extensions, target_dir) in DIRECTORIES_MAP {
        if extensions.iter().any(|&ext| ext == lowercase_extension.as_str()) {
            return Some(target_dir);
        }
    }
    None
}

fn main() {
    //get the download directory
    let download_dir = download_dir().unwrap();

    //check if the destination directories exist within the download directory
    check_directories_existence(&download_dir);

    loop {
        //read the contents of the directory
        let contents = fs::read_dir(&download_dir);

        //iterate through the contents of the directory
        match contents {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        //get the file name
                        let file_name = entry.file_name();

                        //extract the file extension
                        if let Some(file_extension) = get_file_extension(file_name.to_str().unwrap()) {
                            //find the target directory of the file
                            if let Some(target_dir) = find_target_directory(file_extension) {
                                //move the file to the target directory
                                if let Err(err) = fs::rename(entry.path(), download_dir.join(target_dir).join(file_name)) {
                                    //display an error dialog if the move operation fails
                                    MessageDialog::new()
                                        .set_type(MessageType::Error)
                                        .set_title("Error")
                                        .set_text(&format!("Failed to move file: {}", err))
                                        .show_alert().expect("Error displaying dialog");
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                //display an error dialog if reading the directory fails
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("Error")
                    .set_text(&format!("Failed to read directory: {}", err))
                    .show_alert().expect("Error displaying dialog");
            }
        }
        //sleep for some time before checking again (adjust as needed)
        std::thread::sleep(Duration::from_secs(60));
    }
}