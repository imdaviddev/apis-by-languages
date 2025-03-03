-module(server).
-export([start/0]).

start() ->
  rooster:start(#{port => 8080},
                #{routes => [hello()]}).

hello() ->
  {'GET', "/hello", fun(_) -> {200, #{message => <<"hello world">>}} end}.
