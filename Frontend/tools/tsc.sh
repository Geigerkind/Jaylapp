cd "$(dirname "$0")";

NUM_CORES=$1;

FILES="";
LAST_PATHTOFILE="";
for f in $(find ./../ts/ -name "*.ts"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep tsc | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    PATHTOFILE=$(dirname "$f" | cut -c 8-);
    NEWDIR="./../.tmp_js$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    if [ "$PATHTOFILE" != "$LAST_PATHTOFILE" ] && [ "$LAST_PATHTOFILE" != "" ] && [ "$FILES" != "" ]; then
        tsc --alwaysStrict --baseUrl "/" --target "es6" --locale "en" --newLine "lf" --strict --listFiles $FILES --outFile ./../.tmp_js$LAST_PATHTOFILE/merge.js & 
        FILES=$f;
        LAST_PATHTOFILE=$PATHTOFILE;
    else
        FILES="$FILE $f";
        LAST_PATHTOFILE=$PATHTOFILE;
    fi
done
if [ "$FILES" != "" ]; then
    while [ $(ps aux | grep tsc | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    tsc --alwaysStrict --baseUrl "/" --target "es6" --locale "en" --newLine "lf" --strict --listFiles $FILES --outFile ./../.tmp_js$LAST_PATHTOFILE/merge.js & 
fi
