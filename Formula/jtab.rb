# Generated with JReleaser 1.3.1 at 2022-11-11T22:55:50.441376115Z
class Jtab < Formula
  desc "Print any json data as a table from the command line"
  homepage "https://github.com/wlezzar/jtab"
  version "0.7.0"
  license "MIT"

  if OS.linux? && Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.0/jtab-0.7.0-aarch64-unknown-linux-gnu.zip"
    sha256 "97d1e2221d17d502bc7b6904c4d0c33228b0a8b1f56aa389bc4a8894aa871e89"
  end
  if OS.linux? && Hardware::CPU.intel?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.0/jtab-0.7.0-x86_64-unknown-linux-gnu.zip"
    sha256 "47758f8418914727ebaeee021be9d796edf60949852dce72f1d24871d668a433"
  end
  if OS.mac? && Hardware::CPU.arm?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.0/jtab-0.7.0-aarch64-apple-darwin.zip"
    sha256 "771effc9904e6e4fd2c8b30b49325fd365ba51a6d81f1466ff6db68053203fd4"
  end
  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.0/jtab-0.7.0-x86_64-apple-darwin.zip"
    sha256 "a6c3c6b12d970061cb2fee6460752f224542fed69bf80600e68c6f0fdc398e31"
  end


  def install
    libexec.install Dir["*"]
    bin.install_symlink "#{libexec}/bin/jtab" => "jtab"
  end

  test do
    output = shell_output("#{bin}/jtab --version")
    assert_match "0.7.0", output
  end
end
