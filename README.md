### traj-propagate

A command line utility that reads from and writes to SPICE SPK files
to propagate trajectories of specified bodies. The user must provide SPKs containing
position and velocity data of the bodies they wish to include in the calculation so that an initial
condition for the propagation can be assembled from them. If bodies whose standard
gravitational parameter is not contained in the included PCKs need to be included in the
calculations, additional PCKs must be provided. All user-provided kernels must be loaded from
a single meta-kernel text file.

The program will output a single SPK containing the propagated trajectory data for all bodies
that were included in the calculation. This kernel can then be used with other SPICE integrated tools, such
as SPICE-Enhanced Cosmographia for trajectory visualisation.

Note that the [CSPICE library](https://naif.jpl.nasa.gov/naif/toolkit.html) needs to be installed for this program to work. (see [these requirements](https://github.com/gregoirehenry/rust-spice#requirements))

### Usage

```
traj-propagate 0.0.1

pixldemon <moritzamando@protonmail.com>

USAGE:
    traj-propagate [OPTIONS] --mk <FILE> --t0 <UTC_TIMESTAMP> --dt <NUM_DAYS> --h <NUM_MINUTES> --bodies <BODIES>... --output-file <FILE>

OPTIONS:
        --bodies <BODIES>...    Comma-separated NAIF-IDs or body names
        --cb-id <NAIF_ID>       Observing body for SPK segments. Defaults to first body in list
        --dt <NUM_DAYS>         Time period over which to propagate
        --fts <FRACTION>        Fraction of steps to save to SPK file. 1 saves every step, 0.5 every
                                2nd etc. Defaults to 1
        --h <NUM_MINUTES>       Timestep size for integration
    -h, --help                  Print help information
        --mk <FILE>             Meta-kernel file name
    -o, --output-file <FILE>    File to write results to
        --t0 <UTC_TIMESTAMP>    Time at which to begin propagation
    -V, --version               Print version information
```

### Example

```
traj-propagate --mk spice_data/custom.tm --t0 '2013-NOV-19 00:00:00' --h 10 --dt 308 --bodies=Sun,Earth,Mars,5,-202 -o spice_data/maven_cruise.bsp
Propagating interactions of 6 bodies over 308 days (dt=10min)
Progress:  [##################################################] 100% complete
```

This will propagate the trajectory of NASA's MAVEN mission from shortly after launch to just before it reached Mars and save the trajectory to `maven_cruise.bsp`

```
$ brief spice_data/maven_cruise.bsp

BRIEF -- Version 4.0.0, September 8, 2010 -- Toolkit Version N0066


Summary for: spice_data/maven_cruise.bsp

Bodies: MAVEN (-202)            EARTH (399)
        JUPITER BARYCENTER (5)  MARS (499)
        Start of Interval (ET)              End of Interval (ET)
        -----------------------------       -----------------------------
        2013 NOV 19 00:01:07.182            2014 SEP 23 00:01:07.182
```
