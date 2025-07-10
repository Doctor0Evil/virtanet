
#!/usr/bin/perl
use strict;
use warnings;
use utf8;
use Encode;

# Function to escape special characters for regex
sub escape_regex {
    my ($str) = @_;
    $str =~ s/([\\\^\$\.\*\+\?\{\}\(\)\[\]\|])/\\$1/g;
    return $str;
}

# Array of predefined regex patterns with descriptions
my @patterns = (
    {
        name => "Find all .txt files",
        regex => qr/.*\.txt\z/u,
        description => "Matches all files ending with .txt"
    },
    {
        name => "Find all .dat files",
        regex => qr/.*\.dat\z/u,
        description => "Matches all files ending with .dat"
    },
    {
        name => "Numeric file sequence",
        regex => qr/data\.\d{3,4}\z/u,
        description => "Matches files like data.001, data.002, ..."
    },
    {
        name => "Recursive C files",
        regex => qr/.*/.*\.c\z/u,
        description => "Matches all .c files recursively in directories"
    },
    {
        name => "Block relative path exploit",
        regex => qr/(?!.*\.\.\/)/u,
        description => "Blocks usage of ../ in paths"
    },
    {
        name => "Allow safe filename",
        regex => qr/^[a-zA-Z0-9\-\.]+\z/u,
        description => "Allows only safe characters in filenames"
    },
    {
        name => "Block hidden files",
        regex => qr/^\.[^\/]+\z/u,
        description => "Blocks files starting with a dot"
    },
    {
        name => "Find all images",
        regex => qr/.*\.(jpg|jpeg|png|gif|bmp)\z/iu,
        description => "Matches common image file extensions"
    },
    {
        name => "Find all videos",
        regex => qr/.*\.(mp4|avi|mov|mkv)\z/iu,
        description => "Matches common video file extensions"
    },
    {
        name => "Find all audio",
        regex => qr/.*\.(mp3|wav|flac|aac)\z/iu,
        description => "Matches common audio file extensions"
    },
    {
        name => "Find all documents",
        regex => qr/.*\.(pdf|docx?|xlsx?|pptx?)\z/iu,
        description => "Matches common document file extensions"
    },
    {
        name => "Find all archives",
        regex => qr/.*\.(zip|tar|gz|rar|7z)\z/iu,
        description => "Matches common archive file extensions"
    },
    {
        name => "Find all scripts",
        regex => qr/.*\.(sh|bat|ps1|cmd)\z/iu,
        description => "Matches shell and batch script files"
    },
    {
        name => "Find all executables",
        regex => qr/.*\.(exe|bin|out|app)\z/iu,
        description => "Matches executable file extensions"
    },
    {
        name => "Find all markdown files",
        regex => qr/.*\.md\z/u,
        description => "Matches markdown documentation files"
    },
    {
        name => "Find all JSON files",
        regex => qr/.*\.json\z/u,
        description => "Matches JSON data files"
    },
    {
        name => "Find all XML files",
        regex => qr/.*\.xml\z/u,
        description => "Matches XML data files"
    },
    {
        name => "Find all YAML files",
        regex => qr/.*\.(ya?ml)\z/u,
        description => "Matches YAML configuration files"
    },
    {
        name => "Find all CSV files",
        regex => qr/.*\.csv\z/u,
        description => "Matches CSV data files"
    },
    {
        name => "Find all log files",
        regex => qr/.*\.log\z/u,
        description => "Matches log files"
    },
    {
        name => "Find all backup files",
        regex => qr/.*\.(bak|old|backup)\z/iu,
        description => "Matches backup file extensions"
    },
    {
        name => "Find all temp files",
        regex => qr/.*\.(tmp|temp)\z/iu,
        description => "Matches temporary file extensions"
    },
    {
        name => "Find all swap files",
        regex => qr/.*\.(swp|swo)\z/iu,
        description => "Matches swap file extensions"
    },
    {
        name => "Find all core dumps",
        regex => qr/core\.\d+\z/u,
        description => "Matches core dump files"
    },
    {
        name => "Find all Python files",
        regex => qr/.*\.py\z/u,
        description => "Matches Python source files"
    },
    {
        name => "Find all Java files",
        regex => qr/.*\.java\z/u,
        description => "Matches Java source files"
    },
    {
        name => "Find all C++ files",
        regex => qr/.*\.(cpp|cc|cxx|hpp|h)\z/iu,
        description => "Matches C++ source and header files"
    },
    {
        name => "Find all Go files",
        regex => qr/.*\.go\z/u,
        description => "Matches Go source files"
    },
    {
        name => "Find all Rust files",
        regex => qr/.*\.rs\z/u,
        description => "Matches Rust source files"
    },
    {
        name => "Find all TypeScript files",
        regex => qr/.*\.ts\z/u,
        description => "Matches TypeScript source files"
    },
    {
        name => "Find all JavaScript files",
        regex => qr/.*\.js\z/u,
        description => "Matches JavaScript source files"
    },
    {
        name => "Find all HTML files",
        regex => qr/.*\.html?\z/iu,
        description => "Matches HTML files"
    },
    {
        name => "Find all CSS files",
        regex => qr/.*\.css\z/u,
        description => "Matches CSS stylesheet files"
    },
    {
        name => "Find all PHP files",
        regex => qr/.*\.php\z/u,
        description => "Matches PHP source files"
    },
    {
        name => "Find all Ruby files",
        regex => qr/.*\.rb\z/u,
        description => "Matches Ruby source files"
    },
    {
        name => "Find all Perl files",
        regex => qr/.*\.pl\z/u,
        description => "Matches Perl source files"
    },
    {
        name => "Find all Shell files",
        regex => qr/.*\.sh\z/u,
        description => "Matches shell script files"
    },
    {
        name => "Find all Makefiles",
        regex => qr/(?:^|\/)Makefile\z/u,
        description => "Matches Makefile in any directory"
    },
    {
        name => "Find all Dockerfiles",
        regex => qr/(?:^|\/)Dockerfile\z/u,
        description => "Matches Dockerfile in any directory"
    },
    {
        name => "Find all config files",
        regex => qr/.*\.(conf|cfg|ini|config)\z/iu,
        description => "Matches configuration files"
    },
    {
        name => "Find all environment files",
        regex => qr/.*\.env\z/u,
        description => "Matches environment variable files"
    },
    {
        name => "Find all license files",
        regex => qr/(?:^|\/)LICENSE\z/u,
        description => "Matches LICENSE file in any directory"
    },
    {
        name => "Find all readme files",
        regex => qr/(?:^|\/)README(?:\.md)?\z/u,
        description => "Matches README and README.md files"
    },
    {
        name => "Find all requirements files",
        regex => qr/(?:^|\/)requirements\.txt\z/u,
        description => "Matches Python requirements files"
    },
    {
        name => "Find all setup files",
        regex => qr/(?:^|\/)setup\.(?:py|cfg)\z/u,
        description => "Matches Python setup files"
    },
    {
        name => "Find all test files",
        regex => qr/.*test.*\.(?:py|js|ts|cpp|java)\z/iu,
        description => "Matches test files in various languages"
    },
    {
        name => "Find all migration files",
        regex => qr/.*migration.*\.(?:sql|py)\z/iu,
        description => "Matches migration scripts"
    },
    {
        name => "Find all patch files",
        regex => qr/.*\.patch\z/u,
        description => "Matches patch files"
    },
    {
        name => "Find all diff files",
        regex => qr/.*\.diff\z/u,
        description => "Matches diff files"
    },
    {
        name => "Find all lock files",
        regex => qr/.*\.lock\z/u,
        description => "Matches lock files"
    },
    {
        name => "Find all checksum files",
        regex => qr/.*\.(?:md5|sha1|sha256|sha512)\z/iu,
        description => "Matches checksum files"
    },
    {
        name => "Find all certificate files",
        regex => qr/.*\.(?:crt|pem|key|cer)\z/iu,
        description => "Matches certificate and key files"
    },
    {
        name => "Find all font files",
        regex => qr/.*\.(?:ttf|otf|woff|woff2)\z/iu,
        description => "Matches font files"
    },
    {
        name => "Find all favicon files",
        regex => qr/.*favicon\.(?:ico|png)\z/iu,
        description => "Matches favicon files"
    },
    {
        name => "Find all robots.txt",
        regex => qr/(?:^|\/)robots\.txt\z/u,
        description => "Matches robots.txt in any directory"
    },
    {
        name => "Find all sitemap files",
        regex => qr/.*sitemap.*\.xml\z/u,
        description => "Matches sitemap XML files"
    },
    {
        name => "Find all changelog files",
        regex => qr/(?:^|\/)CHANGELOG(?:\.md)?\z/u,
        description => "Matches CHANGELOG and CHANGELOG.md"
    },
    {
        name => "Find all gitignore files",
        regex => qr/(?:^|\/)\.gitignore\z/u,
        description => "Matches .gitignore in any directory"
    },
    {
        name => "Find all gitattributes files",
        regex => qr/(?:^|\/)\.gitattributes\z/u,
        description => "Matches .gitattributes in any directory"
    },
    {
        name => "Find all gitmodules files",
        regex => qr/(?:^|\/)\.gitmodules\z/u,
        description => "Matches .gitmodules in any directory"
    },
    {
        name => "Find all editorconfig files",
        regex => qr/(?:^|\/)\.editorconfig\z/u,
        description => "Matches .editorconfig in any directory"
    },
    {
        name => "Find all npmrc files",
        regex => qr/(?:^|\/)\.npmrc\z/u,
        description => "Matches .npmrc in any directory"
    },
    {
        name => "Find all yarnrc files",
        regex => qr/(?:^|\/)\.yarnrc\z/u,
        description => "Matches .yarnrc in any directory"
    },
    {
        name => "Find all package.json",
        regex => qr/(?:^|\/)package\.json\z/u,
        description => "Matches package.json in any directory"
    },
    {
        name => "Find all package-lock.json",
        regex => qr/(?:^|\/)package-lock\.json\z/u,
        description => "Matches package-lock.json in any directory"
    },
    {
        name => "Find all yarn.lock",
        regex => qr/(?:^|\/)yarn\.lock\z/u,
        description => "Matches yarn.lock in any directory"
    },
    {
        name => "Find all composer.json",
        regex => qr/(?:^|\/)composer\.json\z/u,
        description => "Matches composer.json in any directory"
    },
    {
        name => "Find all composer.lock",
        regex => qr/(?:^|\/)composer\.lock\z/u,
        description => "Matches composer.lock in any directory"
    },
    {
        name => "Find all Gemfile",
        regex => qr/(?:^|\/)Gemfile\z/u,
        description => "Matches Gemfile in any directory"
    },
    {
        name => "Find all Gemfile.lock",
        regex => qr/(?:^|\/)Gemfile\.lock\z/u,
        description => "Matches Gemfile.lock in any directory"
    },
    {
        name => "Find all Pipfile",
        regex => qr/(?:^|\/)Pipfile\z/u,
        description => "Matches Pipfile in any directory"
    },
    {
        name => "Find all Pipfile.lock",
        regex => qr/(?:^|\/)Pipfile\.lock\z/u,
        description => "Matches Pipfile.lock in any directory"
    },
    {
        name => "Find all pyproject.toml",
        regex => qr/(?:^|\/)pyproject\.toml\z/u,
        description => "Matches pyproject.toml in any directory"
    },
    {
        name => "Find all tox.ini",
        regex => qr/(?:^|\/)tox\.ini\z/u,
        description => "Matches tox.ini in any directory"
    },
    {
        name => "Find all pytest.ini",
        regex => qr/(?:^|\/)pytest\.ini\z/u,
        description => "Matches pytest.ini in any directory"
    },
    {
        name => "Find all setup.cfg",
        regex => qr/(?:^|\/)setup\.cfg\z/u,
        description => "Matches setup.cfg in any directory"
    },
    {
        name => "Find all MANIFEST.in",
        regex => qr/(?:^|\/)MANIFEST\.in\z/u,
        description => "Matches MANIFEST.in in any directory"
    },
    {
        name => "Find all .dockerignore",
        regex => qr/(?:^|\/)\.dockerignore\z/u,
        description => "Matches .dockerignore in any directory"
    },
    {
        name => "Find all .env.example",
        regex => qr/(?:^|\/)\.env\.example\z/u,
        description => "Matches .env.example in any directory"
    },
    {
        name => "Find all .env.local",
        regex => qr/(?:^|\/)\.env\.local\z/u,
        description => "Matches .env.local in any directory"
    },
    {
        name => "Find all .env.production",
        regex => qr/(?:^|\/)\.env\.production\z/u,
        description => "Matches .env.production in any directory"
    },
    {
        name => "Find all .env.development",
        regex => qr/(?:^|\/)\.env\.development\z/u,
        description => "Matches .env.development in any directory"
    },
    {
        name => "Find all .bashrc",
        regex => qr/(?:^|\/)\.bashrc\z/u,
        description => "Matches .bashrc in any directory"
    },
    {
        name => "Find all .zshrc",
        regex => qr/(?:^|\/)\.zshrc\z/u,
        description => "Matches .zshrc in any directory"
    },
    {
        name => "Find all .profile",
        regex => qr/(?:^|\/)\.profile\z/u,
        description => "Matches .profile in any directory"
    },
    {
        name => "Find all .bash_profile",
        regex => qr/(?:^|\/)\.bash_profile\z/u,
        description => "Matches .bash_profile in any directory"
    },
    {
        name => "Find all .bash_logout",
        regex => qr/(?:^|\/)\.bash_logout\z/u,
        description => "Matches .bash_logout in any directory"
    },
    {
        name => "Find all .cshrc",
        regex => qr/(?:^|\/)\.cshrc\z/u,
        description => "Matches .cshrc in any directory"
    },
    {
        name => "Find all .tcshrc",
        regex => qr/(?:^|\/)\.tcshrc\z/u,
        description => "Matches .tcshrc in any directory"
    },
    {
        name => "Find all .login",
        regex => qr/(?:^|\/)\.login\z/u,
        description => "Matches .login in any directory"
    },
    {
        name => "Find all .logout",
        regex => qr/(?:^|\/)\.logout\z/u,
        description => "Matches .logout in any directory"
    },
    {
        name => "Find all .vimrc",
        regex => qr/(?:^|\/)\.vimrc\z/u,
        description => "Matches .vimrc in any directory"
    },
    {
        name => "Find all .emacs",
        regex => qr/(?:^|\/)\.emacs\z/u,
        description => "Matches .emacs in any directory"
    },
    {
        name => "Find all .nanorc",
        regex => qr/(?:^|\/)\.nanorc\z/u,
        description => "Matches .nanorc in any directory"
    },
    {
        name => "Find all .screenrc",
        regex => qr/(?:^|\/)\.screenrc\z/u,
        description => "Matches .screenrc in any directory"
    },
    {
        name => "Find all .tmux.conf",
        regex => qr/(?:^|\/)\.tmux\.conf\z/u,
        description => "Matches .tmux.conf in any directory"
    },
    {
        name => "Find all .inputrc",
        regex => qr/(?:^|\/)\.inputrc\z/u,
        description => "Matches .inputrc in any directory"
    },
    {
        name => "Find all .Xresources",
        regex => qr/(?:^|\/)\.Xresources\z/u,
        description => "Matches .Xresources in any directory"
    },
    {
        name => "Find all .Xauthority",
        regex => qr/(?:^|\/)\.Xauthority\z/u,
        description => "Matches .Xauthority in any directory"
    },
    {
        name => "Find all .xinitrc",
        regex => qr/(?:^|\/)\.xinitrc\z/u,
        description => "Matches .xinitrc in any directory"
    },
    {
        name => "Find all .xsession",
        regex => qr/(?:^|\/)\.xsession\z/u,
        description => "Matches .xsession in any directory"
    },
    {
        name => "Find all .xsessionrc",
        regex => qr/(?:^|\/)\.xsessionrc\z/u,
        description => "Matches .xsessionrc in any directory"
    },
    {
        name => "Find all .gtkrc",
        regex => qr/(?:^|\/)\.gtkrc\z/u,
        description => "Matches .gtkrc in any directory"
    },
    {
        name => "Find all .gtkrc-2.0",
        regex => qr/(?:^|\/)\.gtkrc-2\.0\z/u,
        description => "Matches .gtkrc-2.0 in any directory"
    },
    {
        name => "Find all .config directories",
        regex => qr/(?:^|\/)\.config(?:\/)?\z/u,
        description => "Matches .config directories"
    },
    {
        name => "Find all .local directories",
        regex => qr/(?:^|\/)\.local(?:\/)?\z/u,
        description => "Matches .local directories"
    },
    {
        name => "Find all .cache directories",
        regex => qr/(?:^|\/)\.cache(?:\/)?\z/u,
        description => "Matches .cache directories"
    },
    {
        name => "Find all .mozilla directories",
        regex => qr/(?:^|\/)\.mozilla(?:\/)?\z/u,
        description => "Matches .mozilla directories"
    },
    {
        name => "Find all .thunderbird directories",
        regex => qr/(?:^|\/)\.thunderbird(?:\/)?\z/u,
        description => "Matches .thunderbird directories"
    },
    {
        name => "Find all .ssh directories",
        regex => qr/(?:^|\/)\.ssh(?:\/)?\z/u,
        description => "Matches .ssh directories"
    },
    {
        name => "Find all .gnupg directories",
        regex => qr/(?:^|\/)\.gnupg(?:\/)?\z/u,
        description => "Matches .gnupg directories"
    },
    {
        name => "Find all .aws directories",
        regex => qr/(?:^|\/)\.aws(?:\/)?\z/u,
        description => "Matches .aws directories"
    },
    {
        name => "Find all .azure directories",
        regex => qr/(?:^|\/)\.azure(?:\/)?\z/u,
        description => "Matches .azure directories"
    },
    {
        name => "Find all .gcloud directories",
        regex => qr/(?:^|\/)\.gcloud(?:\/)?\z/u,
        description => "Matches .gcloud directories"
    },
    {
        name => "Find all .kube directories",
        regex => qr/(?:^|\/)\.kube(?:\/)?\z/u,
        description => "Matches .kube directories"
    },
    {
        name => "Find all .docker directories",
        regex => qr/(?:^|\/)\.docker(?:\/)?\z/u,
        description => "Matches .docker directories"
    },
    {
        name => "Find all .npm directories",
        regex => qr/(?:^|\/)\.npm(?:\/)?\z/u,
        description => "Matches .npm directories"
    },
    {
        name => "Find all .yarn directories",
        regex => qr/(?:^|\/)\.yarn(?:\/)?\z/u,
        description => "Matches .yarn directories"
    },
    {
        name => "Find all .cache/yarn",
        regex => qr/(?:^|\/)\.cache\/yarn(?:\/)?\z/u,
        description => "Matches .cache/yarn directories"
    },
    {
        name => "Find all .vscode directories",
        regex => qr/(?:^|\/)\.vscode(?:\/)?\z/u,
        description => "Matches .vscode directories"
    },
    {
        name => "Find all .idea directories",
        regex => qr/(?:^|\/)\.idea(?:\/)?\z/u,
        description => "Matches .idea directories"
    },
    {
        name => "Find all .ipynb files",
        regex => qr/.*\.ipynb\z/u,
        description => "Matches Jupyter notebook files"
    },
    {
        name => "Find all .Rmd files",
        regex => qr/.*\.Rmd\z/u,
        description => "Matches R Markdown files"
    },
    {
        name => "Find all .r files",
        regex => qr/.*\.r\z/u,
        description => "Matches R source files"
    },
    {
        name => "Find all .mat files",
        regex => qr/.*\.mat\z/u,
        description => "Matches MATLAB data files"
    },
    {
        name => "Find all .m files",
        regex => qr/.*\.m\z/u,
        description => "Matches MATLAB/Objective-C files"
    },
    {
        name => "Find all .sas files",
        regex => qr/.*\.sas\z/u,
        description => "Matches SAS script files"
    },
    {
        name => "Find all .sav files",
        regex => qr/.*\.sav\z/u,
        description => "Matches SPSS data files"
    },
    {
        name => "Find all .dta files",
        regex => qr/.*\.dta\z/u,
        description => "Matches Stata data files"
    },
    {
        name => "Find all .sql files",
        regex => qr/.*\.sql\z/u,
        description => "Matches SQL script files"
    },
    {
        name => "Find all .db files",
        regex => qr/.*\.db\z/u,
        description => "Matches SQLite database files"
    },
    {
        name => "Find all .sqlite files",
        regex => qr/.*\.sqlite\z/u,
        description => "Matches SQLite database files"
    },
    {
        name => "Find all .mdb files",
        regex => qr/.*\.mdb\z/u,
        description => "Matches Microsoft Access database files"
    },
    {
        name => "Find all .accdb files",
        regex => qr/.*\.accdb\z/u,
        description => "Matches Microsoft Access database files"
    },
    {
        name => "Find all .bak files",
        regex => qr/.*\.bak\z/u,
        description => "Matches backup files"
    },
   {
        name => "Find all .tar.gz files",
        regex => qr/.*\.tar\.gz\z/u,
        description => "Matches tarball gzip archives"
    },
    {
        name => "Find all .tgz files",
        regex => qr/.*\.tgz\z/u,
        description => "Matches tgz compressed files"
    },
    {
        name => "Find all .tar.bz2 files",
        regex => qr/.*\.tar\.bz2\z/u,
        description => "Matches tarball bzip2 archives"
    },
    {
        name => "Find all .tbz2 files",
        regex => qr/.*\.tbz2\z/u,
        description => "Matches tbz2 compressed files"
    },
);

# Example usage:
my $string_to_test = ".env.production";

foreach my $pattern (@patterns) {
    if ($string_to_test =~ $pattern->{regex}) {
        print "String '$string_to_test' matches pattern: " . $pattern->{name} . "\n";
        print "Description: " . $pattern->{description} . "\n";
    }
}

1;
```
