### traj-propagate
This program is a command line utility that reads from and writes to SPICE SPK kernel files
to propagate trajectories of the specified bodies. The user must provide SPK kernels containing
position and velocity data of the bodies they wish to include in the calculation so that an initial
condition for the propagation can be assembled from them. If bodies whose standard
gravitational parameter is not contained in the included PCK kernels need to be included in the 
calculations, additional PCK kernels must be provided. All user-provided kernels must be loaded from
a single meta-kernel text file.

The program will output a single SPK kernel containing the propagated trajectory data for all bodies
that were included in the calculation. This kernel can be used with other SPICE integrated tools. The
trajectories could be visualised using SPICE-Enhanced Cosmographia, for example.

Note that the [CSPICE library](https://naif.jpl.nasa.gov/naif/toolkit.html) needs to be installed on your system for this program to compile (see [these requirements](https://github.com/gregoirehenry/rust-spice#requirements))

### Usage
```
$ traj-propagate --help

traj-propagate 0.0.1

pixldemon <moritzamando@protonmail.com>

USAGE:
    traj-propagate [OPTIONS] --mk <FILE> --t0 <UTC_TIMESTAMP> --time <NUM_DAYS> --dt <NUM_MINUTES> --output-file <FILE>

OPTIONS:
        --bodies <BODY_LIST>    String containing comma-separated NAIF-IDs or body names
        --cb-id <NAIF_ID>       Observing body for SPK segments. Defaults to first body in list
        --dt <NUM_MINUTES>      Timestep size for numerical integration
    -h, --help                  Print help information
        --mk <FILE>             Meta-kernel file name
    -o, --output-file <FILE>    File to write results to
        --sts <NUM_STEPS>       Number of steps to skip between each saved one to reduce output file
                                size. Defaults to 0
        --t0 <UTC_TIMESTAMP>    Time at which to begin propagation
        --time <NUM_DAYS>       Time period over which to integrate
    -V, --version               Print version information
```

### Example
```
$ traj-propagate --mk spice_data/custom.tm --t0 '2013-NOV-19 00:00:00' --dt 10 --time 308 --bodies 'Sun, Earth, Mars, 5, -202' -o spice_data/maven_cruise.bsp
Propagating interactions of 5 bodies over 308 days (dt=10min)
Progress:  [##################################################] 100% complete 
```
This will propagate the trajectory of NASA's MAVEN mission from shortly after launch to just before it reached Mars and save the trajectory to `maven_cruise.bsp`

```
$ brief spice_data/maven_cruise.bsp

BRIEF -- Version 4.0.0, September 8, 2010 -- Toolkit Version N0066
 
 
Summary for: spice_data/maven_cruise.bsp
 
Bodies: MAVEN (-202)            JUPITER BARYCENTER (5)  EARTH (399)
        Start of Interval (ET)              End of Interval (ET)
        -----------------------------       -----------------------------
        2013 NOV 19 00:01:07.182            2014 SEP 23 00:01:07.182
```