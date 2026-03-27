class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.2.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "4d190cf0a0ecd7ef81add42fdce6b3087feb02e9e4c30b4b732fc0efa76651b5"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "e632b8c3f46ab77012cbb045c1d192577f0b5da318811986a7613363edaece24"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "0d6e7bee775c75595621c93ac71613e591acf9066560cf9abc83cc5dcf02f454"
  end

  def install
    bin.install "glow"
    bin.install "glw"

    generate_completions_from_executable(bin/"glow", "completions")
  end

  test do
    assert_match "Glow CLI", shell_output("#{bin}/glow --help")
    assert_match version.to_s, shell_output("#{bin}/glow --version")
    assert_match "Glow CLI", shell_output("#{bin}/glw --help")
  end
end
