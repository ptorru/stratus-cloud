if [ $# -eq 1 ]
then
    if  [ $1 == "clean" ]
    then
        cargo clean
    fi
fi

PATH="$(pwd)/raspberrypi_tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH" cargo build --bin helloworld-server --target arm-unknown-linux-gnueabihf

if [ $# -eq 1 ]
then
    SEND="scp target/arm-unknown-linux-gnueabihf/release/oled_ssd1306_rpi $PIUSR@$IPADD:"

    if  [ $1 == "send" ]
    then
        $SEND
    fi

    if  [ $1 == "run" ]
    then
        $SEND
        echo "Running..."
        ssh $PIUSR@$IPADD "./oled_ssd1306_rpi"
        echo "Return code (should be 0): $?"
    fi
fi
