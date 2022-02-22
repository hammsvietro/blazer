# Blazer

Blazer is a blazingly fast case parser for maps, strings and JSON using NIFs.

Useful to consume camel/pascal cased APIs while still using elixir's snake case convention

Uses [Jason](https://hexdocs.pm/jason/readme.html) to parse the json data.

## Installation

It can be installed by adding `blazer` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:blazer, "~> 0.1.1"}
  ]
end
```


# Basic Usage

```elixir
iex(1)> Blazer.parse(%{"categoryId" => 13, "profileName" => "hammsvietro"}, case: :snake)
{:ok, %{category_id: 13, profile_name: "hammsvietro"}}


iex(2)> Jason.encode!(%{"category_id" => 13, "profile_name" => "hammsvietro"}, case: :camel))
"{\"categoryId\":13,\"profileName\":\"hammsvietro\"}"


```
# Usage with Phoenix
In your config.exs:
```elixir

config :blazer, inner_case: :snake, outer_case: :camel
config :phoenix, :json_library, Blazer
```
There you go! Now every map key will use elixir's standard snake case while externally the API will produce/consume camel cased properties


# Contributing
Contributions are very welcome! Feel free to open an issue =)

# License
Blazer source code is licensed under the [MIT License](LICENSE).
