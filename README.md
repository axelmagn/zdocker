# zDocker

zDocker is a simple docker utility intended to simplify the management of
projects with multiple docker-compose configurations.  Because of course we
need another layer on top of docker. It is intended to be exceedingly simple,
allowing users to define environments consisting of multiple docker-compose
configuration files in which to execute docker-compose commands.

## Usage

    Usage:
        zdocker [options] <envs> [--] <args>...

    Options:
        -h --help           Show this screen.
        --version           Show version.
        -c --config=<conf>  The config file to use [default: ./.zdocker.toml].

## Configuration

zdocker checks the current working directory for a .zdocker.toml file that
defines different environments.
