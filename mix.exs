defmodule Bubblegum.MixProject do
  use Mix.Project

  def project do
    [
      app: :bubblegum,
      version: "0.1.0",
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      aliases: aliases()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.36.1", runtime: false},
      {:ex_doc, "~> 0.30.0", only: :dev, runtime: false},
      {:dialyxir, "~> 1.3", only: [:dev], runtime: false}
    ]
  end

  defp rustler_crates do
    [
      bubblegum_nif: [
        path: "native/bubblegum_nif",
        mode: if(Mix.env() == :prod, do: :release, else: :debug),
        load_from: {:bubblegum, "priv/native/libbubblegum"},
        crate: :bubblegum_nif,
        default_features: true,
        features: ["solana"]
      ]
    ]
  end

  defp aliases do
    [
      test: ["compile", "test"],
      "compile.rustler": ["cmd --cd native/bubblegum_nif cargo build --verbose"]
    ]
  end
end