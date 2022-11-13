# Generated with JReleaser 1.3.1 at 2022-11-13T02:16:34.36797186Z
class Jtab < Formula
  desc "Print any json data as a table from the command line"
  homepage "https://github.com/wlezzar/jtab"
  version "0.7.1"
  license "MIT"

  if OS.linux? && Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.1/jtab-0.7.1-aarch64-unknown-linux-gnu.zip"
    sha256 "aa31f30f7bd419be66f1971e3bdffb0f251135cf36941017d850d504e51b1b74"
  end
  if OS.linux? && Hardware::CPU.intel?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.1/jtab-0.7.1-x86_64-unknown-linux-gnu.zip"
    sha256 "05b7f99d948c10e056fa3d1e3d7a71b462d2fcd4ea8caeb5799f5c5aa59fbc42"
  end
  if OS.mac? && Hardware::CPU.arm?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.1/jtab-0.7.1-aarch64-apple-darwin.zip"
    sha256 "1c323399f59dfb8583cdc8402819ee6e77b0719a23e6d5e609a281a423c87a23"
  end
  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/wlezzar/jtab/releases/download/v0.7.1/jtab-0.7.1-x86_64-apple-darwin.zip"
    sha256 "f6861e1512037eb39093c216e33d25bcc79e7bd6ba47cbfd37f06de1f7ab1111"
  end


  def install
    libexec.install Dir["*"]
    bin.install_symlink "#{libexec}/bin/jtab" => "jtab"
  end

  test do
    output = shell_output("#{bin}/jtab --version")
    assert_match "0.7.1", output
  end
end
