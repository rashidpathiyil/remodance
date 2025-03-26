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
          "artifacts/**/*.dmg",
          "artifacts/**/*.deb",
          "artifacts/**/*.AppImage",
          "artifacts/**/*.rpm",
          "artifacts/**/*.msi",
          "artifacts/**/*.exe",
          "artifacts/**/*.pdb",
          "artifacts/**/*.json",
          "artifacts/**/remodance*"
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
