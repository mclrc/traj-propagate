<img width="128" height="128" src="/logo.png" />
<h1>traj-propagate</h1>

![test workflow](https://github.com/mclrc/traj-propagate/actions/workflows/tests.yml/badge.svg)


Command line utility that reads from and writes to SPICE SPK files to propagate trajectories for spacecraft, planets or other bodies.

Initial conditions must be given in the form of a single kernel (this can be a meta-kernel) from which the state of all specified bodies at `t0` can be retrieved. For large bodies whose standard gravitational parameter is not given in the included kernels, additional PCKs must be provided. For small bodies, no additional data is required.

The program will output a single new SPK containing all propagated trajectories. This kernel can then be used with other SPICE integrated tools, such as SPICE-Enhanced Cosmographia for trajectory visualisation.

Note that the [CSPICE library](https://naif.jpl.nasa.gov/naif/toolkit.html) needs to be installed for this program to work. (see [these requirements](https://github.com/gregoirehenry/rust-spice#requirements))

## Usage

```
traj-propagate 0.0.1
pixldemon <moritzamando@protonmail.com>

USAGE:
    traj-propagate [OPTIONS] --mk <FILE> --t0 <UTC_TIMESTAMP> --tfinal <UTC_TIMESTAMP> --h <NUM_MINUTES> --output-file <FILE>

OPTIONS:
        --atol <ATOL>
            Error tolerance for embedded methods (target magnitude of error estimate)
        --attractors <ATTRACTORS>...
            Bodies whose states to pull from SPICE instead of propagating
        --bodies <BODIES>...
            Large bodies whose gravitational influence to consider and whose trajectories to propagate
        --cb-id <NAIF_ID>
            Observing body for SPK segments. Defaults to first body in list
        --fts <FRACTION>
            Fraction of steps to save to SPK file. 1 saves every step, 0.5 every 2nd etc. Defaults
            to 1
        --h <NUM_MINUTES>
            Timestep size for integration
    -h, --help
            Print help information
        --method <rk4|dopri45>
            Integration method
        --mk <FILE>
            Meta-kernel file name
    -o, --output-file <FILE>
            File to write results to
        --small-bodies <SMALL_BODIES>...
            Bodies to include whose gravitational pull/mass can be ignored (e. g. spacecraft)
        --t0 <UTC_TIMESTAMP>
            Time at which to begin propagation
        --tfinal <UTC_TIMESTAMP>
            J2000 time to propagate up to
    -V, --version
            Print version information
```
## Example

```
traj-propagate --mk spice/tests.tm \
  --t0 '2013-NOV-20' --tfinal 2014-SEP-20 \
  --cb-id=10 --bodies=Sun,Earth,5,499 --small-bodies=-202 \
  --method dopri45 --h 1000 --fts 1 --atol 10000 \
  -o example.bsp
```

This will propagate the trajectory of NASA's MAVEN probe from shortly after launch to just before it reached Mars and save the trajectory to `example.bsp`

```
$ brief -c example.bsp

BRIEF -- Version 4.1.0, September 17, 2021 -- Toolkit Version N0067
 
 
Summary for: example.bsp
 
Bodies: MAVEN (-202) w.r.t. SUN (10)
        JUPITER BARYCENTER (5) w.r.t. SUN (10)
        EARTH (399) w.r.t. SUN (10)
        MARS (499) w.r.t. SUN (10)
        Start of Interval (ET)              End of Interval (ET)
        -----------------------------       -----------------------------
        2013 NOV 20 00:17:47.182            2014 SEP 20 00:01:07.182
```
