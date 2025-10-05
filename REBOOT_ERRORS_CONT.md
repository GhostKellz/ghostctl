 mesa-git is up to date
:: Starting full system upgrade...
warning: kwin-polonium: local (1.0rc-1.12) is newer than chaotic-aur (1.0rc-1.3)
resolving dependencies...
looking for conflicting packages...

Package (5)                      Old Version   New Version  Net Change  Download Size

extra/haskell-pandoc             3.4-5         3.4-6          0.04 MiB       8.02 MiB
extra/haskell-pandoc-lua-engine  0.3.2-5       0.3.2-6        0.00 MiB       0.59 MiB
extra/haskell-pandoc-server      0.1.0.8-7     0.1.0.8-8      0.00 MiB       0.10 MiB
extra/haskell-texmath            0.12.8.10-19  0.12.8.11-1   -0.07 MiB       3.82 MiB
extra/pandoc-cli                 3.4-7         3.4-8          0.00 MiB       0.11 MiB

Total Download Size:   12.64 MiB
Total Installed Size:  90.68 MiB
Net Upgrade Size:      -0.02 MiB

:: Proceed with installation? [Y/n]
:: Retrieving packages...
 haskell-texmath-0.12.8.11-1-x...     3.8 MiB  73.5 MiB/s 00:00 [##################################] 100%
 haskell-pandoc-server-0.1.0.8...   106.3 KiB  1329 KiB/s 00:00 [##################################] 100%
 pandoc-cli-3.4-8-x86_64            108.6 KiB  1341 KiB/s 00:00 [##################################] 100%
 haskell-pandoc-lua-engine-0.3...   606.6 KiB  4.20 MiB/s 00:00 [##################################] 100%
 haskell-pandoc-3.4-6-x86_64          8.0 MiB  24.6 MiB/s 00:00 [##################################] 100%
 Total (5/5)                         12.6 MiB  28.8 MiB/s 00:00 [##################################] 100%
(5/5) checking keys in keyring                                  [##################################] 100%
(5/5) checking package integrity                                [##################################] 100%
(5/5) loading package files                                     [##################################] 100%
(5/5) checking for file conflicts                               [##################################] 100%
(5/5) checking available disk space                             [##################################] 100%
:: Running pre-transaction hooks...
(1/2) Performing snapper pre snapshots for the following configurations...
==> root: 1730
(2/2) Unregistering Haskell modules...
:: Processing package changes...
(1/5) upgrading haskell-texmath                                 [##################################] 100%
(2/5) upgrading haskell-pandoc                                  [##################################] 100%
(3/5) upgrading haskell-pandoc-lua-engine                       [##################################] 100%
(4/5) upgrading haskell-pandoc-server                           [##################################] 100%
(5/5) upgrading pandoc-cli                                      [##################################] 100%
:: Running post-transaction hooks...
(1/3) Arming ConditionNeedsUpdate...
(2/3) Registering Haskell modules...
(3/3) Performing snapper post snapshots for the following configurations...
==> root: 1731
 -> 1 error occurred:
        * request failed: Get "https://aur.archlinux.org/rpc?arg%5B%5D=bfg&arg%5B%5D=cervisia&arg%5B%5D=curseforge&arg%5B%5D=dxvk-mingw&arg%5B%5D=github-desktop-bin&arg%5B%5D=gnu-netcat&arg%5B%5D=kdesu5&arg%5B%5D=kinit&arg%5B%5D=lib32-vulkan-tools&arg%5B%5D=linux-cachyos-lto&arg%5B%5D=linux-cachyos-lto-headers&arg%5B%5D=linux-ghost&arg%5B%5D=linux-ghost-headers&arg%5B%5D=microsoft-edge-dev-bin&arg%5B%5D=neofetch&arg%5B%5D=vagrant&arg%5B%5D=wazuh-agent&arg%5B%5D=wine-gaming-dependencies&arg%5B%5D=yay-debug&arg%5B%5D=zsh-you-should-use&type=info&v=5": read tcp 10.0.0.21:60086->65.108.85.17:443: read: connection reset by peer


❯ sudo pacman -Syy
:: Synchronizing package databases...
 core                               120.4 KiB  1606 KiB/s 00:00 [##################################] 100%
 extra                                8.0 MiB  31.8 MiB/s 00:00 [##################################] 100%
 multilib                           136.8 KiB  1777 KiB/s 00:00 [##################################] 100%
 chaotic-aur                        649.1 KiB   487 KiB/s 00:01 [##################################] 100%
 proaudio                            30.9 KiB  56.5 KiB/s 00:01 [##################################] 100%
 liquorix                          1101.0   B  13.8 KiB/s 00:00 [##################################] 100%
 python                            1444.0   B  30.0 KiB/s 00:00 [##################################] 100%
 mesa-git                             8.5 KiB  25.8 KiB/s 00:00 [##################################] 100%
❯
❯ sudo yay
❯ yay -Syy
:: Synchronizing package databases...
 core                               120.4 KiB  1627 KiB/s 00:00 [##################################] 100%
 extra                                8.0 MiB  25.9 MiB/s 00:00 [##################################] 100%
 multilib                           136.8 KiB  1754 KiB/s 00:00 [##################################] 100%
 chaotic-aur                        649.1 KiB   492 KiB/s 00:01 [##################################] 100%
 proaudio                            30.9 KiB  55.8 KiB/s 00:01 [##################################] 100%
 liquorix                          1101.0   B  16.0 KiB/s 00:00 [##################################] 100%
 python                            1444.0   B  29.4 KiB/s 00:00 [##################################] 100%
 mesa-git                             8.5 KiB  26.9 KiB/s 00:00 [##################################] 100%

    ~                                                    󰊠 GhostKellz.sh 󰊠   
