class GlowBeta < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.9.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.9.1-beta/glow-aarch64-apple-darwin.tar.gz"
      sha256 "475804d3f6eb8df2a20cce9be4f196dde73e6ce71041ef624ce3a95f3dbf9213"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.9.1-beta/glow-x86_64-apple-darwin.tar.gz"
      sha256 "83faa028ed156ade59894689afeac345dd5428ff91f1f056b19ed49ab0af6f89"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.9.1-beta/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "1fd82ebad3c28646d3dfc6a5f8777dcf0e988ff5cf21b2be830be5a20d502224"
  end

  def install
    bin.install "glow-beta"
    bin.install "glw-beta"

    generate_completions_from_executable(bin/"glow-beta", "completions")
  end

  test do
    assert_match "Glow CLI", shell_output("#{bin}/glow-beta --help")
    assert_match version.to_s, shell_output("#{bin}/glow-beta --version")
    assert_match "Glow CLI", shell_output("#{bin}/glw-beta --help")
  end
end
