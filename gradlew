#!/bin/sh
#
# Gradle wrapper script for POSIX systems — Gradle 8.6
# SPDX-License-Identifier: Apache-2.0
#

# Resolve APP_HOME
app_path=$0
while [ -h "$app_path" ] ; do
    ls=$( ls -ld "$app_path" )
    link=${ls#*' -> '}
    case $link in
      /*)   app_path=$link ;;
      *)    app_path=${app_path%"${app_path##*/}"}$link ;;
    esac
done
APP_HOME=$( cd "${app_path%"${app_path##*/}"}." && pwd -P ) || exit

APP_NAME="Gradle"
APP_BASE_NAME=${0##*/}
DEFAULT_JVM_OPTS='"-Xmx64m" "-Xms64m"'
MAX_FD=maximum
CLASSPATH=$APP_HOME/gradle/wrapper/gradle-wrapper.jar

warn () { echo "$*"; } >&2
die ()  { echo; echo "$*"; echo; exit 1; } >&2

cygwin=false; darwin=false; msys=false; nonstop=false
case "$( uname )" in
  CYGWIN* )  cygwin=true  ;;
  Darwin* )  darwin=true  ;;
  MSYS*|MINGW*) msys=true ;;
  NONSTOP* ) nonstop=true ;;
esac

if [ -n "$JAVA_HOME" ] ; then
    if [ -x "$JAVA_HOME/jre/sh/java" ] ; then
        JAVACMD=$JAVA_HOME/jre/sh/java
    else
        JAVACMD=$JAVA_HOME/bin/java
    fi
    [ ! -x "$JAVACMD" ] && die "ERROR: JAVA_HOME points to an invalid directory: $JAVA_HOME"
else
    JAVACMD=java
    command -v java >/dev/null 2>&1 || die "ERROR: JAVA_HOME is not set and 'java' was not found in PATH."
fi

if ! "$cygwin" && ! "$darwin" && ! "$nonstop" ; then
    case $MAX_FD in
      max*) MAX_FD=$( ulimit -H -n ) || warn "Could not query max file descriptors" ;;
    esac
    case $MAX_FD in
      ''|soft) :;;
      *) ulimit -n "$MAX_FD" || warn "Could not set max file descriptors to $MAX_FD" ;;
    esac
fi

eval set -- $DEFAULT_JVM_OPTS $JAVA_OPTS $GRADLE_OPTS \
    "\"-Dorg.gradle.appname=$APP_BASE_NAME\"" \
    -classpath "\"$CLASSPATH\"" \
    org.gradle.wrapper.GradleWrapperMain "$@"

exec "$JAVACMD" "$@"
