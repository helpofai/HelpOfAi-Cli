# Integration — CLI Commands

## Module Management
```
hoa module list                      → list all modules with status
hoa module info <id>                 → module details
hoa module load <id>                 → load specific module
hoa module unload <id>               → unload specific module
hoa module reload <id>               → reload module
```

## Registry
```
hoa registry refresh                 → reload registry from disk
hoa registry dump                    → dump full registry to stdout
hoa registry stats                   → module/capability counts
```

## Profile
```
hoa profile list                     → available profiles
hoa profile use <name>               → switch profile
hoa profile create <name>            → create new profile
hoa profile delete <name>            → delete profile
```

## Plugin
```
hoa plugin list                      → list installed plugins
hoa plugin load <dir>                → load plugin from directory
hoa plugin unload <id>               → unload plugin
```

## Debug
```
hoa integration debug --load-trace   → trace module loading
hoa integration debug --profile      → integration performance stats
```