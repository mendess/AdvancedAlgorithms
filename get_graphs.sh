#!/bin/bash
BASE="http://data.law.di.unimi.it/webdata"
BASE_DIR=graphs
EXTS=(
    -hc-t.graph
    -hc-t.properties
    -hc.graph
    -hc.properties
    -nat.fcl
    -nat.graph
    -nat.properties
    -nat.urls.gz
    -t.graph
    -t.properties
    .fcl
    .graph
    .indegree
    .lmap
    .map
    .md5sums
    .outdegree
    .properties
    .scc
    .sccsizes
    .smap
    .urls.gz
    .stats
)

GRAPHS=(
    cnr-2000
    in-2004
    eu-2005
    uk-2007-05@100000
    uk-2007-05@1000000
)

PAR_JOBS=0
CLASSPATH="$CLASSPATH:/home/mendess/tmp/jar"
for f in /home/mendess/tmp/jar/t/*; do
    CLASSPATH="$CLASSPATH:$f"
done
echo "$CLASSPATH" | tr ':' '\n'
export CLASSPATH
for g in "${GRAPHS[@]}"; do
    for ext in "${EXTS[@]}"; do
        file="$BASE_DIR/$g/${g}${ext}"
        [ -e "$file" ] || {
            echo -e "\e[33mDownloading:\e[0m $file" &&
                wget --quiet -P "$BASE_DIR/$g" -c "${BASE}/${g}/${g}${ext}" &&
                echo -e "\e[32mSaved:\e[0m $file" ||
                echo -e "\e[31mError:\e[0m $file"
        } &

        ((PAR_JOBS++))
        while [ "$PAR_JOBS" -ge 8 ]; do
            wait -n
            ((PAR_JOBS--))
        done
    done
done
wait
for g in "${GRAPHS[@]}"; do
    cd "$BASE_DIR/$g" || exit
    md5sum -c "$g.md5sums"
    java it.unimi.dsi.webgraph.BVGraph -o -O -L "$g"
    java it.unimi.dsi.webgraph.examples.BreadthFirst "$g"
    read -p "press enter"
    cd - &>/dev/null || exit
done
