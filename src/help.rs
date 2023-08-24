pub fn help() -> String {
    String::from(
"NAME
    exportbranch - export branch
SYNOPSIS
    exportbranch -s <source> -d <destination> [-c <only_copy_files>] [-f <file_filters>] [--md5] [--reload]
DESCRIPTION
    Export branch from source to destination
    -s <source>             Source path
    -d <destination>        Destination path
    -c <only_copy_files>    Only copy files
    -f <file_filters>       File filters
    --md5                   Reload files
    --reload                Reload files too
    --lower                 Lowercase files
    Example: exportbranch -s /home/user/branch -d /home/user/branch2 -c *.prg;*.mke;*.mkp;*.mks;*.mkc;*.hbp;*.hbc;*.hbm;*.ch;*.so*;*.cpp;*.a;*.c;*.h;*.sh;*.0;*.18;*.jar;*.spec -f *.a;*.so;*.h;*.0;*.18;*.jar;*.spec"
)
}
