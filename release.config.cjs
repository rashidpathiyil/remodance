module.exports = {
  branches: ["main"],
  repositoryUrl: "https://github.com/rashidpathiyil/remodance.git",
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/github",
      {
        assets: [
          "release-assets/*.dmg",
          "release-assets/*.deb",
          "release-assets/*.AppImage",
          "release-assets/*.rpm",
          "release-assets/*.msi",
          "release-assets/*.exe",
          "release-assets/*.pdb",
          "release-assets/*.json",
          "release-assets/remodance*"
        ],
      },
    ],
    [
      "@semantic-release/git",
      {
        assets: ["package.json", "src-tauri/tauri.conf.json"],
        message: "chore(release): ${nextRelease.version}\n\n${nextRelease.notes}",
      },
    ],
  ],
};
