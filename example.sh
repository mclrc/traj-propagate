cargo run --release -- --mk spice/tests.tm \
  --t0 '2013-NOV-20' --tfinal 2014-SEP-20 \
  --cb-id=10 --bodies=Sun,Earth,5,499 --small-bodies=-202 \
  --method dopri45 --h 1000 --fts 1 --atol 10000 \
  -o example.bsp
