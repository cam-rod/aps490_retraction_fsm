EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr B 17000 11000
encoding utf-8
Sheet 1 1
Title "ST_LINK_V2-1-SCHDOC"
Date "22 01 2024"
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Notes 10160 10980 0    60   ~ 0
3
Text Notes 10410 10980 0    60   ~ 0
3
Text Notes 8560 10520 0    60   ~ 12
STLINK/V2-1
Text Notes 9280 10830 0    60   ~ 0
MB1180
Text Notes 10280 10830 0    60   ~ 0
July-2002: Re-released for DXP Platform.
Text Notes 8550 10980 0    60   ~ 0
*
Wire Notes Line
	8300 10550 10700 10550
Wire Notes Line
	10700 10700 8300 10700
Wire Notes Line
	8800 10850 8800 10700
Wire Notes Line
	9850 11000 9850 10700
Wire Notes Line
	10700 11000 10700 10400
Text Notes 8330 10520 0    60   ~ 0
Title:
Text Notes 8330 10830 0    60   ~ 0
Size:
Text Notes 8850 10830 0    60   ~ 0
Reference:
Text Notes 8330 10980 0    60   ~ 0
Date:
Text Notes 9900 10980 0    60   ~ 0
Sheet:
Text Notes 10290 10980 0    60   ~ 0
of
Text Notes 8550 10830 0    60   ~ 0
A4
Wire Notes Line
	10700 10850 8300 10850
Text Notes 9900 10830 0    60   ~ 0
Revision:
Text Notes 8640 10670 0    60   ~ 12
NUCLEO32
Text Notes 8330 10670 0    60   ~ 0
Project:
Wire Notes Line
	8300 11000 8300 10400
Wire Notes Line
	8300 10400 11500 10400
$Comp
L power:GND #PWR?65AEE5B0
U 1 1 65AEE5B0
P 1000 5400
F 0 "GND_17" H 1000 5540 20  0000 C CNN
F 1 "GND" H 1000 5510 30  0000 C CNN
F 2 "" H 1000 5400 70  0000 C CNN
F 3 "" H 1000 5400 70  0000 C CNN
	1    1000 5400
	1    0    0    -1  
$EndComp
Wire Wire Line
	3200 7900 3200 7600
Wire Wire Line
	3300 7900 3300 7600
Wire Wire Line
	3400 7900 3400 7600
Wire Wire Line
	5400 6700 4900 6700
Wire Wire Line
	5600 6600 4900 6600
Wire Wire Line
	5400 6100 4900 6100
Wire Wire Line
	5400 6000 4900 6000
Text Label 4900 6100 0 54 ~
USB_DM
Text Label 4900 6000 0 54 ~
USB_DP
Text Label 1800 6300 0 54 ~
STM_RST
Text Label 3200 7900 1 54 ~
T_JTCK
Text Label 4900 6700 0 54 ~
T_JTCK
Text Label 3300 7900 1 54 ~
T_JTDO
Text Label 3400 7900 1 54 ~
T_JTDI
Text Label 4900 6600 0 54 ~
T_JTMS
Wire Wire Line
	5400 5900 4900 5900
Wire Wire Line
	4100 4300 4100 4900
Text Label 4900 5900 0 54 ~
STM_JTMS
Text Label 4100 4900 1 54 ~
STM_JTCK
Wire Wire Line
	4100 7700 4100 7600
Wire Wire Line
	5100 5700 4900 5700
Wire Wire Line
	4000 8000 4000 7600
$Comp
L power:GND #PWR?65AEE5AF
U 1 1 65AEE5AF
P 700 6800
F 0 "GND_18" H 700 6940 20  0000 C CNN
F 1 "GND" H 700 6910 30  0000 C CNN
F 2 "" H 700 6800 70  0000 C CNN
F 3 "" H 700 6800 70  0000 C CNN
	1    700 6800
	1    0    0    -1  
$EndComp
Text Label 1800 6100 0 54 ~
OSC_IN
Text Label 1800 6200 0 54 ~
OSC_OUT
Wire Wire Line
	3700 8000 3700 7600
Wire Wire Line
	5400 6300 4900 6300
Wire Wire Line
	3500 7900 3500 7600
Text Label 3500 7900 1 54 ~
T_NRST
Text Label 2000 6600 0 54 ~
AIN_1
Wire Wire Line
	3400 4300 3400 4900
Wire Wire Line
	1500 6100 2200 6100
Wire Wire Line
	1500 5700 1500 6100
Wire Wire Line
	1000 6200 2200 6200
Wire Wire Line
	1000 5700 1000 6200
$Comp
L power:GND #PWR?65AEE5AE
U 1 1 65AEE5AE
P 2200 6400
F 0 "GND_19" H 2200 6540 20  0000 C CNN
F 1 "GND" H 2200 6510 30  0000 C CNN
F 2 "" H 2200 6400 70  0000 C CNN
F 3 "" H 2200 6400 70  0000 C CNN
	1    2200 6400
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?65AEE5AD
U 1 1 65AEE5AD
P 3000 4300
F 0 "GND_20" H 3000 4440 20  0000 C CNN
F 1 "GND" H 3000 4410 30  0000 C CNN
F 2 "" H 3000 4300 70  0000 C CNN
F 3 "" H 3000 4300 70  0000 C CNN
	1    3000 4300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3100 4800 3100 4900
Wire Wire Line
	3000 4800 3100 4800
Wire Wire Line
	3000 4300 3000 4800
$Comp
L power:GND #PWR?65AEE5AC
U 1 1 65AEE5AC
P 5400 5800
F 0 "GND_21" H 5400 5940 20  0000 C CNN
F 1 "GND" H 5400 5910 30  0000 C CNN
F 2 "" H 5400 5800 70  0000 C CNN
F 3 "" H 5400 5800 70  0000 C CNN
	1    5400 5800
	1    0    0    -1  
$EndComp
Wire Wire Line
	4900 5800 5400 5800
$Comp
L power:GND #PWR?65AEE5AB
U 1 1 65AEE5AB
P 4000 8000
F 0 "GND_22" H 4000 8140 20  0000 C CNN
F 1 "GND" H 4000 8110 30  0000 C CNN
F 2 "" H 4000 8000 70  0000 C CNN
F 3 "" H 4000 8000 70  0000 C CNN
	1    4000 8000
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?65AEE5AA
U 1 1 65AEE5AA
P 3700 8000
F 0 "GND_23" H 3700 8140 20  0000 C CNN
F 1 "GND" H 3700 8110 30  0000 C CNN
F 2 "" H 3700 8000 70  0000 C CNN
F 3 "" H 3700 8000 70  0000 C CNN
	1    3700 8000
	1    0    0    -1  
$EndComp
Text Notes 1000 9200 0    84   ~ 12
USB ST-LINK
Wire Wire Line
	2200 9400 2200 9600
$Comp
L power:+U5V #PWR?65AEE5A9
U 1 1 65AEE5A9
P 2200 9400
F 0 "U5V" H 2200 9400 20  0000 C CNN
F 1 "U5V" H 2200 9330 30  0000 C CNN
F 2 "" H 2200 9400 70  0000 C CNN
F 3 "" H 2200 9400 70  0000 C CNN
	1    2200 9400
	1    0    0    -1  
$EndComp
Text Notes 8400 5700 0    84   ~ 12
COM
Text Notes 9000 9200 0    84   ~ 12
PWR
$Comp
L power:GND #PWR?65AEE5A8
U 1 1 65AEE5A8
P 6500 4500
F 0 "GND_24" H 6500 4640 20  0000 C CNN
F 1 "GND" H 6500 4610 30  0000 C CNN
F 2 "" H 6500 4500 70  0000 C CNN
F 3 "" H 6500 4500 70  0000 C CNN
	1    6500 4500
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?65AEE5A7
U 1 1 65AEE5A7
P 1300 4300
F 0 "GND_25" H 1300 4440 20  0000 C CNN
F 1 "GND" H 1300 4410 30  0000 C CNN
F 2 "" H 1300 4300 70  0000 C CNN
F 3 "" H 1300 4300 70  0000 C CNN
	1    1300 4300
	1    0    0    -1  
$EndComp
Text Notes 300 4500 0    60   ~ 0
Board Ident: PC13=0
Wire Wire Line
	2000 5800 2200 5800
Wire Wire Line
	2000 4300 2000 5800
Wire Wire Line
	1800 4300 2000 4300
Wire Wire Line
	1900 5900 2200 5900
Wire Wire Line
	1900 4600 1900 5900
Wire Wire Line
	1800 4600 1900 4600
Text Label 8000 3900 0 54 ~
T_JTCK
Text Label 8000 4000 0 54 ~
T_JTMS
Text Notes 8100 4900 0    72   ~ 12
SWCLK
Text Notes 8100 4800 0    72   ~ 12
SWDIO
Wire Wire Line
	10300 3900 8000 3900
Wire Wire Line
	8000 4000 10300 4000
Text Label 5000 7000 0 54 ~
T_SWDIO_IN
Wire Wire Line
	4900 7000 4900 6800
Wire Wire Line
	5600 7000 4900 7000
Text Label 4900 6300 0 54 ~
LED_STLINK
Text Label 8300 6100 0 54 ~
LED_STLINK
$Comp
L power:GND #PWR?65AEE5A6
U 1 1 65AEE5A6
P 10200 5700
F 0 "GND_26" H 10200 5840 20  0000 C CNN
F 1 "GND" H 10200 5810 30  0000 C CNN
F 2 "" H 10200 5700 70  0000 C CNN
F 3 "" H 10200 5700 70  0000 C CNN
	1    10200 5700
	1    0    0    -1  
$EndComp
Wire Wire Line
	9800 5700 10200 5700
NoConn ~ 2200 6700
NoConn ~ 2200 6000
NoConn ~ 3100 7600
NoConn ~ 3800 4900
Wire Wire Line
	2200 4900 3000 4900
Wire Wire Line
	2200 5700 2200 4900
Wire Wire Line
	1800 6600 1900 6600
Wire Wire Line
	2200 5000 2300 5000
Wire Wire Line
	1000 5400 1500 5400
Text GLabel 10300 4000 2 0 BiDi ~
TMS
Text GLabel 10300 3900 2 0 BiDi ~
TCK
Text Notes 8600 3900 0    60   ~ 0
TCK/SWCLK
Text Notes 8600 4000 0    60   ~ 0
TMS/SWDIO
Text Label 6600 6400 0 54 ~
MCO
Wire Wire Line
	5400 6400 4900 6400
Text GLabel 6800 6400 2 0 BiDi ~
MCO
Text Notes 3600 7900 1    60   ~ 0
T_JRST
NoConn ~ 3600 7600
Text GLabel 10300 4100 2 0 BiDi ~
NRST
Text Label 8000 4100 0 54 ~
T_NRST
Wire Wire Line
	700 6300 2200 6300
Wire Wire Line
	6800 6400 5800 6400
$Comp
L power:GND #PWR?65AEE5A5
U 1 1 65AEE5A5
P 5800 6000
F 0 "GND_27" H 5800 6140 20  0000 C CNN
F 1 "GND" H 5800 6110 30  0000 C CNN
F 2 "" H 5800 6000 70  0000 C CNN
F 3 "" H 5800 6000 70  0000 C CNN
	1    5800 6000
	1    0    0    -1  
$EndComp
Text Label 2400 7900 0 60 ~
STLINK_RX
Wire Wire Line
	3000 7900 2000 7900
Wire Wire Line
	3000 7600 3000 7900
Wire Wire Line
	900 7900 1600 7900
Wire Wire Line
	900 7500 1600 7500
Text GLabel 300 7900 2 0 Input ~
STLK_RX
Text GLabel 300 7500 2 0 Output ~
STLK_TX
Text Label 2100 7400 1 60 ~
STLINK_TX
Wire Wire Line
	2100 7500 2000 7500
Wire Wire Line
	2100 6800 2100 7500
Wire Wire Line
	2200 6800 2100 6800
Text Label 3000 9700 0 54 ~
USB_DM
Text Label 3000 9800 0 54 ~
USB_DP
Wire Wire Line
	3500 9700 2200 9700
Wire Wire Line
	3500 9800 2200 9800
Wire Wire Line
	2300 9600 2300 9800
Wire Wire Line
	2400 9600 2300 9600
Wire Wire Line
	2400 9900 2200 9900
Text Label 4900 6200 0 54 ~
T_SWO
Wire Wire Line
	4900 6200 5400 6200
Text GLabel 10300 4200 2 0 BiDi ~
SWO
Text Label 8000 4200 0 54 ~
T_SWO
Wire Wire Line
	10300 4200 8000 4200
$Comp
L power:+U5V #PWR?65AEE5A4
U 1 1 65AEE5A4
P 7700 9400
F 0 "U5V_2" H 7700 9400 20  0000 C CNN
F 1 "U5V" H 7700 9330 30  0000 C CNN
F 2 "" H 7700 9400 70  0000 C CNN
F 3 "" H 7700 9400 70  0000 C CNN
	1    7700 9400
	1    0    0    -1  
$EndComp
Wire Wire Line
	3200 9600 2800 9600
Text Label 4300 9300 0 60 ~
USB_RENUMn
Text Label 4000 4900 1 54 ~
USB_RENUMn
Wire Wire Line
	4000 4300 4000 4900
Wire Wire Line
	3900 3800 3900 4900
Wire Wire Line
	3500 3800 3300 3800
$Comp
L power:GND #PWR?65AEE5A3
U 1 1 65AEE5A3
P 4500 3800
F 0 "GND_28" H 4500 3940 20  0000 C CNN
F 1 "GND" H 4500 3910 30  0000 C CNN
F 2 "" H 4500 3800 70  0000 C CNN
F 3 "" H 4500 3800 70  0000 C CNN
	1    4500 3800
	1    0    0    -1  
$EndComp
Wire Wire Line
	4500 3800 4300 3800
$Comp
L power:++3V3_ST_LINK #PWR?65AEE5A2
U 1 1 65AEE5A2
P 10200 6100
F 0 "+3V3_ST_LINK" H 10200 6100 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 10200 6030 30  0000 C CNN
F 2 "" H 10200 6100 70  0000 C CNN
F 3 "" H 10200 6100 70  0000 C CNN
	1    10200 6100
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE5A1
U 1 1 65AEE5A1
P 6500 4000
F 0 "+3V3_ST_LINK_2" H 6500 4000 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 6500 3930 30  0000 C CNN
F 2 "" H 6500 4000 70  0000 C CNN
F 3 "" H 6500 4000 70  0000 C CNN
	1    6500 4000
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE5A0
U 1 1 65AEE5A0
P 5100 5700
F 0 "+3V3_ST_LINK_3" H 5100 5700 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 5100 5630 30  0000 C CNN
F 2 "" H 5100 5700 70  0000 C CNN
F 3 "" H 5100 5700 70  0000 C CNN
	1    5100 5700
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE59F
U 1 1 65AEE59F
P 4100 7700
F 0 "+3V3_ST_LINK_4" H 4100 7700 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 4100 7630 30  0000 C CNN
F 2 "" H 4100 7700 70  0000 C CNN
F 3 "" H 4100 7700 70  0000 C CNN
	1    4100 7700
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE59E
U 1 1 65AEE59E
P 700 5900
F 0 "+3V3_ST_LINK_5" H 700 5900 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 700 5830 30  0000 C CNN
F 2 "" H 700 5900 70  0000 C CNN
F 3 "" H 700 5900 70  0000 C CNN
	1    700 5900
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE59D
U 1 1 65AEE59D
P 2300 5000
F 0 "+3V3_ST_LINK_6" H 2300 5000 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 2300 4930 30  0000 C CNN
F 2 "" H 2300 5000 70  0000 C CNN
F 3 "" H 2300 5000 70  0000 C CNN
	1    2300 5000
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE59C
U 1 1 65AEE59C
P 3300 4100
F 0 "+3V3_ST_LINK_7" H 3300 4100 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 3300 4030 30  0000 C CNN
F 2 "" H 3300 4100 70  0000 C CNN
F 3 "" H 3300 4100 70  0000 C CNN
	1    3300 4100
	1    0    0    -1  
$EndComp
Wire Wire Line
	3300 4100 3500 4100
Text Notes 3900 4900 1    60   ~ 0
PWR_EXT
$Comp
L power:++3V3_ST_LINK #PWR?65AEE59B
U 1 1 65AEE59B
P 1400 6500
F 0 "+3V3_ST_LINK_8" H 1400 6500 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 1400 6430 30  0000 C CNN
F 2 "" H 1400 6500 70  0000 C CNN
F 3 "" H 1400 6500 70  0000 C CNN
	1    1400 6500
	1    0    0    -1  
$EndComp
Wire Wire Line
	1400 6500 2200 6500
$Comp
L power:VO #PWR?65AEE59A
U 1 1 65AEE59A
P 1400 6600
F 0 "VO_2" H 1400 6600 20  0000 C CNN
F 1 "VO" H 1400 6530 30  0000 C CNN
F 2 "" H 1400 6600 70  0000 C CNN
F 3 "" H 1400 6600 70  0000 C CNN
	1    1400 6600
	1    0    0    -1  
$EndComp
Wire Wire Line
	7700 9400 7900 9400
Wire Wire Line
	7700 9000 7900 9000
Wire Wire Line
	8400 9400 8400 9000
$Comp
L power:GND #PWR?65AEE599
U 1 1 65AEE599
P 8400 10100
F 0 "GND_29" H 8400 10240 20  0000 C CNN
F 1 "GND" H 8400 10210 30  0000 C CNN
F 2 "" H 8400 10100 70  0000 C CNN
F 3 "" H 8400 10100 70  0000 C CNN
	1    8400 10100
	1    0    0    -1  
$EndComp
Wire Wire Line
	9200 10100 9200 9800
Wire Wire Line
	10000 10100 10000 9800
Wire Wire Line
	9900 9400 10000 9400
$Comp
L power:++3V3_ST_LINK #PWR?65AEE598
U 1 1 65AEE598
P 10300 9400
F 0 "+3V3_ST_LINK_9" H 10300 9400 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 10300 9330 30  0000 C CNN
F 2 "" H 10300 9400 70  0000 C CNN
F 3 "" H 10300 9400 70  0000 C CNN
	1    10300 9400
	1    0    0    -1  
$EndComp
$Comp
L power:+U5V #PWR?65AEE597
U 1 1 65AEE597
P 3700 8900
F 0 "U5V_3" H 3700 8900 20  0000 C CNN
F 1 "U5V" H 3700 8830 30  0000 C CNN
F 2 "" H 3700 8900 70  0000 C CNN
F 3 "" H 3700 8900 70  0000 C CNN
	1    3700 8900
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?65AEE596
U 1 1 65AEE596
P 3700 9700
F 0 "GND_30" H 3700 9840 20  0000 C CNN
F 1 "GND" H 3700 9810 30  0000 C CNN
F 2 "" H 3700 9700 70  0000 C CNN
F 3 "" H 3700 9700 70  0000 C CNN
	1    3700 9700
	1    0    0    -1  
$EndComp
Wire Wire Line
	3500 9300 3900 9300
Wire Wire Line
	4800 9300 4300 9300
$Comp
L power:++3V3_ST_LINK #PWR?65AEE595
U 1 1 65AEE595
P 3200 9000
F 0 "+3V3_ST_LINK_10" H 3200 9000 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 3200 8930 30  0000 C CNN
F 2 "" H 3200 9000 70  0000 C CNN
F 3 "" H 3200 9000 70  0000 C CNN
	1    3200 9000
	1    0    0    -1  
$EndComp
$Comp
L power:+E5V #PWR?65AEE594
U 1 1 65AEE594
P 7700 9000
F 0 "E5V_3" H 7700 9000 20  0000 C CNN
F 1 "E5V" H 7700 8930 30  0000 C CNN
F 2 "" H 7700 9000 70  0000 C CNN
F 3 "" H 7700 9000 70  0000 C CNN
	1    7700 9000
	1    0    0    -1  
$EndComp
$Comp
L power:+E5V #PWR?65AEE593
U 1 1 65AEE593
P 3300 3800
F 0 "E5V_4" H 3300 3800 20  0000 C CNN
F 1 "E5V" H 3300 3730 30  0000 C CNN
F 2 "" H 3300 3800 70  0000 C CNN
F 3 "" H 3300 3800 70  0000 C CNN
	1    3300 3800
	1    0    0    -1  
$EndComp
$Comp
L power:+U5V #PWR?65AEE592
U 1 1 65AEE592
P 7000 7200
F 0 "U5V_4" H 7000 7200 20  0000 C CNN
F 1 "U5V" H 7000 7130 30  0000 C CNN
F 2 "" H 7000 7200 70  0000 C CNN
F 3 "" H 7000 7200 70  0000 C CNN
	1    7000 7200
	1    0    0    -1  
$EndComp
Text Label 10200 8000 0 60 ~
Ilim = 510mA  Isc= 1.2Ilim to 1.5Ilim = 612mA to 765mA
$Comp
L power:+U5V_ST_LINK #PWR?65AEE591
U 1 1 65AEE591
P 10100 7600
F 0 "U5V_ST_LINK_2" H 10100 7600 20  0000 C CNN
F 1 "U5V_ST_LINK" H 10100 7530 30  0000 C CNN
F 2 "" H 10100 7600 70  0000 C CNN
F 3 "" H 10100 7600 70  0000 C CNN
	1    10100 7600
	1    0    0    -1  
$EndComp
Wire Wire Line
	7000 7500 7000 7200
Wire Wire Line
	9800 7900 9600 7900
Wire Wire Line
	10100 8500 10100 7900
Wire Wire Line
	9800 8500 10100 8500
Wire Wire Line
	7400 8500 9600 8500
Wire Wire Line
	7400 7900 7400 8500
$Comp
L power:GND #PWR?65AEE590
U 1 1 65AEE590
P 7800 8500
F 0 "GND_31" H 7800 8640 20  0000 C CNN
F 1 "GND" H 7800 8610 30  0000 C CNN
F 2 "" H 7800 8500 70  0000 C CNN
F 3 "" H 7800 8500 70  0000 C CNN
	1    7800 8500
	1    0    0    -1  
$EndComp
Wire Wire Line
	6000 6500 4900 6500
Wire Wire Line
	6000 8200 6000 6500
Wire Wire Line
	7000 8200 6000 8200
Text Label 4900 6500 0 60 ~
PWR_ENn
NoConn ~ 8100 7900
NoConn ~ 3800 7600
NoConn ~ 3900 7600
Wire Wire Line
	700 6800 700 6600
Wire Wire Line
	1400 6800 700 6800
Wire Wire Line
	6500 4000 6500 4100
Wire Wire Line
	6900 4000 6500 4000
Wire Wire Line
	6900 4100 6900 4000
Wire Wire Line
	6500 4500 6500 4400
Wire Wire Line
	6900 4500 6500 4500
Wire Wire Line
	6900 4400 6900 4500
Wire Wire Line
	10100 7600 9800 7600
Wire Wire Line
	1900 6800 1800 6800
Wire Wire Line
	1900 6600 1900 6800
Wire Wire Line
	2200 6600 1900 6600
Wire Wire Line
	8900 6100 8900 5700
Wire Wire Line
	8200 6100 8900 6100
Wire Wire Line
	1300 4300 1400 4300
Wire Wire Line
	1300 4600 1300 4300
Wire Wire Line
	1400 4600 1300 4600
Wire Wire Line
	10300 9400 10300 9800
Wire Wire Line
	10000 9400 10300 9400
Wire Wire Line
	10000 9500 10000 9400
Wire Wire Line
	8400 10100 10300 10100
Wire Wire Line
	8400 9800 8400 10100
Wire Wire Line
	8800 9400 8800 9800
Wire Wire Line
	8400 9400 8800 9400
Wire Wire Line
	8400 9500 8400 9400
Wire Wire Line
	5800 6400 5800 6300
Wire Wire Line
	7000 8200 7000 7900
Wire Wire Line
	8100 8200 7000 8200
Wire Wire Line
	9800 7700 9600 7700
Wire Wire Line
	9800 7600 9800 7700
Wire Wire Line
	9600 7600 9800 7600
Wire Wire Line
	9800 8500 9800 8300
Wire Wire Line
	9600 8500 9800 8500
Wire Wire Line
	9600 8200 9600 8500
Wire Wire Line
	7400 7300 7000 7300
Wire Wire Line
	7400 7600 7400 7300
Text Notes 9400 4600 0    84   ~ 12
SWD
Wire Wire Line
	8600 4900 9200 4900
Wire Wire Line
	8600 4800 9200 4800
Wire Wire Line
	10400 4800 10000 4800
$Comp
L power:GND #PWR?65AEE58F
U 1 1 65AEE58F
P 10400 4800
F 0 "GND_32" H 10400 4940 20  0000 C CNN
F 1 "GND" H 10400 4910 30  0000 C CNN
F 2 "" H 10400 4800 70  0000 C CNN
F 3 "" H 10400 4800 70  0000 C CNN
	1    10400 4800
	1    0    0    -1  
$EndComp
$Comp
L power:++3V3_ST_LINK #PWR?65AEE58E
U 1 1 65AEE58E
P 10200 4700
F 0 "+3V3_ST_LINK_11" H 10200 4700 20  0000 C CNN
F 1 "+3V3_ST_LINK" H 10200 4630 30  0000 C CNN
F 2 "" H 10200 4700 70  0000 C CNN
F 3 "" H 10200 4700 70  0000 C CNN
	1    10200 4700
	1    0    0    -1  
$EndComp
Wire Wire Line
	10200 4900 10200 4700
Wire Wire Line
	10000 4900 10200 4900
Text Label 8600 4800 0 54 ~
STM_JTMS
Text Label 8600 4900 0 54 ~
STM_JTCK
$Comp
L power:GND #PWR?65AEE58D
U 1 1 65AEE58D
P 2200 10800
F 0 "GND_33" H 2200 10940 20  0000 C CNN
F 1 "GND" H 2200 10910 30  0000 C CNN
F 2 "" H 2200 10800 70  0000 C CNN
F 3 "" H 2200 10800 70  0000 C CNN
	1    2200 10800
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 9900 2800 9900
Wire Wire Line
	3000 10700 3000 9900
Wire Wire Line
	2200 10700 3000 10700
Wire Wire Line
	2200 10000 2200 10700
Wire Wire Line
	8000 4100 8600 4100
Wire Wire Line
	10300 4100 9000 4100
Wire Wire Line
	8100 7600 8100 7700
Wire Wire Line
	7400 7600 8100 7600
Wire Wire Line
	2200 10800 2200 10700
Connection ~ 700 6800
Connection ~ 700 6300
Connection ~ 1000 5800
Connection ~ 1000 5400
Connection ~ 1300 4300
Connection ~ 1500 5800
Connection ~ 1900 6600
Connection ~ 2200 10700
Connection ~ 2200 10600
Connection ~ 2200 10500
Connection ~ 2200 10400
Connection ~ 2200 10300
Connection ~ 2200 10200
Connection ~ 2200 5000
Connection ~ 2300 9800
Connection ~ 3000 4300
Connection ~ 3700 9300
Connection ~ 3900 4100
Connection ~ 3900 3800
Connection ~ 5800 6400
Connection ~ 6500 4500
Connection ~ 6500 4000
Connection ~ 7000 8200
Connection ~ 7000 7300
Connection ~ 7400 7600
Connection ~ 7800 8500
Connection ~ 8100 7600
Connection ~ 8400 10100
Connection ~ 8400 9400
Connection ~ 8800 10100
Connection ~ 8800 9600
Connection ~ 8800 9400
Connection ~ 8900 6100
Connection ~ 9200 10100
Connection ~ 9500 10100
Connection ~ 9600 8500
Connection ~ 9800 8500
Connection ~ 9800 7600
Connection ~ 10000 10100
Connection ~ 10000 9400
Connection ~ 10100 7600
Connection ~ 10300 9400
$Comp
L XTAL X2
U 1 1 65AEE58C
P 1200 5700
F 0 "X2" H 1190 5680 60  0000 L BNN
F 1 "NX3225GD 8MHz EXS00A-CG04874" H 1000 5400 60  0000 L BNN
F 2 "" H 1000 5400 60  0000 C CNN
F 3 "" H 1000 5400 60  0000 C CNN
	1    1200 5700
	1    0    0    -1
$EndComp
$Comp
L Res1 R5
U 1 1 65AEE58B
P 5500 6300
F 0 "R5" H 5400 6200 60  0000 L BNN
F 1 "100" H 5600 6200 60  0000 L BNN
F 2 "" H 5600 6200 60  0000 C CNN
F 3 "" H 5600 6200 60  0000 C CNN
	1    5500 6300
	1    0    0    -1
$EndComp
$Comp
L Res1 R20
U 1 1 65AEE58A
P 5500 6900
F 0 "R20" V 5600 6700 60  0000 R TNN
F 1 "100" V 5500 6700 60  0000 R TNN
F 2 "" H 5500 6700 60  0000 C CNN
F 3 "" H 5500 6700 60  0000 C CNN
	1    5500 6900
	0    -1   -1   0
$EndComp
$Comp
L Res1 R13
U 1 1 65AEE589
P 1500 4500
F 0 "R13" H 1400 4400 60  0000 L BNN
F 1 "10K[N/A]" H 1600 4400 60  0000 L BNN
F 2 "" H 1600 4400 60  0000 C CNN
F 3 "" H 1600 4400 60  0000 C CNN
	1    1500 4500
	1    0    0    -1
$EndComp
$Comp
L Res1 R9
U 1 1 65AEE588
P 600 6200
F 0 "R9" V 800 6300 60  0000 R TNN
F 1 "100K" V 700 6300 60  0000 R TNN
F 2 "" H 700 6300 60  0000 C CNN
F 3 "" H 700 6300 60  0000 C CNN
	1    600 6200
	0    -1   -1   0
$EndComp
$Comp
L Res1 R6
U 1 1 65AEE587
P 3100 4200
F 0 "R6" H 3090 4130 60  0000 L BNN
F 1 "100K" H 3090 3970 60  0000 L BNN
F 2 "" H 3090 3970 60  0000 C CNN
F 3 "" H 3090 3970 60  0000 C CNN
	1    3100 4200
	1    0    0    -1
$EndComp
$Comp
L Res1 R16
U 1 1 65AEE586
P 1500 4200
F 0 "R16" H 1400 4100 60  0000 L BNN
F 1 "10K" H 1700 4100 60  0000 L BNN
F 2 "" H 1700 4100 60  0000 C CNN
F 3 "" H 1700 4100 60  0000 C CNN
	1    1500 4200
	1    0    0    -1
$EndComp
$Comp
L Res1 R14
U 1 1 65AEE585
P 1500 6500
F 0 "R14" H 1400 6400 60  0000 L BNN
F 1 "4K7" H 1700 6400 60  0000 L BNN
F 2 "" H 1700 6400 60  0000 C CNN
F 3 "" H 1700 6400 60  0000 C CNN
	1    1500 6500
	1    0    0    -1
$EndComp
$Comp
L Res1 R12
U 1 1 65AEE584
P 1500 6700
F 0 "R12" H 1400 6600 60  0000 L BNN
F 1 "4K7" H 1700 6600 60  0000 L BNN
F 2 "" H 1700 6600 60  0000 C CNN
F 3 "" H 1700 6600 60  0000 C CNN
	1    1500 6700
	1    0    0    -1
$EndComp
$Comp
L Capacitor_ST C2
U 1 1 65AEE583
P 6400 4300
F 0 "C2" V 6410 4110 60  0000 R TNN
F 1 "100nF" V 6310 4110 60  0000 R TNN
F 2 "" H 6310 4110 60  0000 C CNN
F 3 "" H 6310 4110 60  0000 C CNN
	1    6400 4300
	0    -1   -1   0
$EndComp
$Comp
L Capacitor_ST C5
U 1 1 65AEE582
P 6800 4300
F 0 "C5" V 6810 4110 60  0000 R TNN
F 1 "100nF" V 6710 4110 60  0000 R TNN
F 2 "" H 6710 4110 60  0000 C CNN
F 3 "" H 6710 4110 60  0000 C CNN
	1    6800 4300
	0    -1   -1   0
$EndComp
$Comp
L Circuit_Breaker SB3
U 1 1 65AEE581
P 1700 7400
F 0 "SB3" H 1709 7170 60  0000 L BNN
	1    1700 7400
	1    0    0    -1
$EndComp
$Comp
L Capacitor_ST C3
U 1 1 65AEE580
P 5700 6200
F 0 "C3" V 5710 6010 60  0000 R TNN
F 1 "20pF[N/A]" V 5610 6010 60  0000 R TNN
F 2 "" H 5610 6010 60  0000 C CNN
F 3 "" H 5610 6010 60  0000 C CNN
	1    5700 6200
	0    -1   -1   0
$EndComp
$Comp
L Capacitor_ST C21
U 1 1 65AEE57F
P 1400 5600
F 0 "C21" V 1500 5400 60  0000 R TNN
F 1 "10pF" V 1400 5400 60  0000 R TNN
F 2 "" H 1400 5400 60  0000 C CNN
F 3 "" H 1400 5400 60  0000 C CNN
	1    1400 5600
	0    -1   -1   0
$EndComp
$Comp
L Capacitor_ST C20
U 1 1 65AEE57E
P 900 5600
F 0 "C20" V 1000 5800 60  0000 R TNN
F 1 "10pF" V 900 5800 60  0000 R TNN
F 2 "" H 900 5800 60  0000 C CNN
F 3 "" H 900 5800 60  0000 C CNN
	1    900 5600
	0    -1   -1   0
$EndComp
$Comp
L Capacitor_ST C4
U 1 1 65AEE57D
P 600 6500
F 0 "C4" V 500 6400 60  0000 R TNN
F 1 "100nF" V 400 6400 60  0000 R TNN
F 2 "" H 400 6400 60  0000 C CNN
F 3 "" H 400 6400 60  0000 C CNN
	1    600 6500
	0    -1   -1   0
$EndComp
$Comp
L Res1 R11
U 1 1 65AEE57C
P 3600 3900
F 0 "R11" H 3610 4030 60  0000 R BNN
F 1 "2K7" H 3610 3870 60  0000 R BNN
F 2 "" H 3610 3870 60  0000 C CNN
F 3 "" H 3610 3870 60  0000 C CNN
	1    3600 3900
	-1    0    0    -1
$EndComp
$Comp
L Res1 R10
U 1 1 65AEE57B
P 4000 3900
F 0 "R10" H 4010 4030 60  0000 R BNN
F 1 "4K7" H 4010 3870 60  0000 R BNN
F 2 "" H 4010 3870 60  0000 C CNN
F 3 "" H 4010 3870 60  0000 C CNN
	1    4000 3900
	-1    0    0    -1
$EndComp
$Comp
L BAT60JFILM D1
U 1 1 65AEE57A
P 8100 9300
F 0 "D1" H 8090 9310 60  0000 L BNN
F 1 "BAT60JFILM" H 7800 9000 60  0000 L BNN
F 2 "" H 7800 9000 60  0000 C CNN
F 3 "" H 7800 9000 60  0000 C CNN
	1    8100 9300
	1    0    0    -1
$EndComp
$Comp
L BAT60JFILM D2
U 1 1 65AEE579
P 8100 8900
F 0 "D2" H 8090 8910 60  0000 L BNN
F 1 "BAT60JFILM" H 7800 8600 60  0000 L BNN
F 2 "" H 7800 8600 60  0000 C CNN
F 3 "" H 7800 8600 60  0000 C CNN
	1    8100 8900
	1    0    0    -1
$EndComp
$Comp
L CAP_SMD_0603__X5R_1µF_25V_10% C18
U 1 1 65AEE578
P 8400 9800
F 0 "C18" V 8510 9710 60  0000 R TNN
F 1 "1uF_X5R_0603" V 8400 9900 60  0000 R TNN
F 2 "" H 8400 9900 60  0000 C CNN
F 3 "" H 8400 9900 60  0000 C CNN
	1    8400 9800
	0    -1   -1   0
$EndComp
$Comp
L CAP_SMD_0603__X7R_10000PF_50V_10% C17
U 1 1 65AEE577
P 9500 10100
F 0 "C17" V 9610 10010 60  0000 R TNN
F 1 "10nF_X7R_0603" V 9510 10010 60  0000 R TNN
F 2 "" H 9510 10010 60  0000 C CNN
F 3 "" H 9510 10010 60  0000 C CNN
	1    9500 10100
	0    -1   -1   0
$EndComp
$Comp
L Circuit_Breaker SB2
U 1 1 65AEE576
P 1700 7800
F 0 "SB2" H 1709 7570 60  0000 L BNN
	1    1700 7800
	1    0    0    -1
$EndComp
$Comp
L CAP_SMD_0603__X5R_1µF_25V_10% C16
U 1 1 65AEE575
P 10000 9500
F 0 "C16" V 10190 9590 60  0000 L BNN
F 1 "1uF_X5R_0603" V 10290 9590 60  0000 L BNN
F 2 "" H 10290 9590 60  0000 C CNN
F 3 "" H 10290 9590 60  0000 C CNN
	1    10000 9500
	0    1    1    0
$EndComp
$Comp
L LD3985M33R U4
U 1 1 65AEE574
P 9300 9400
F 0 "U4" H 9000 9500 60  0000 L BNN
F 1 "LD3985M33R" H 9200 9500 60  0000 L BNN
F 2 "" H 9200 9500 60  0000 C CNN
F 3 "" H 9200 9500 60  0000 C CNN
	1    9300 9400
	1    0    0    -1
$EndComp
$Comp
L Capacitor_ST C15
U 1 1 65AEE573
P 8700 10000
F 0 "C15" V 8710 9810 60  0000 R TNN
F 1 "100nF" V 8610 9810 60  0000 R TNN
F 2 "" H 8610 9810 60  0000 C CNN
F 3 "" H 8610 9810 60  0000 C CNN
	1    8700 10000
	0    -1   -1   0
$EndComp
$Comp
L Capacitor_ST C19
U 1 1 65AEE572
P 10200 10000
F 0 "C19" V 10210 9810 60  0000 R TNN
F 1 "100nF" V 10110 9810 60  0000 R TNN
F 2 "" H 10110 9810 60  0000 C CNN
F 3 "" H 10110 9810 60  0000 C CNN
	1    10200 10000
	0    -1   -1   0
$EndComp
$Comp
L NPN T1
U 1 1 65AEE571
P 3300 9200
F 0 "T1" H 3700 9100 60  0000 R BNN
F 1 "9013" H 3700 9000 60  0000 R BNN
F 2 "" H 3700 9000 60  0000 C CNN
F 3 "" H 3700 9000 60  0000 C CNN
	1    3300 9200
	-1    0    0    -1
$EndComp
$Comp
L Res1 R4
U 1 1 65AEE570
P 3600 9200
F 0 "R4" V 3710 9070 60  0000 R TNN
F 1 "10K" V 3610 9070 60  0000 R TNN
F 2 "" H 3610 9070 60  0000 C CNN
F 3 "" H 3610 9070 60  0000 C CNN
	1    3600 9200
	0    -1   -1   0
$EndComp
$Comp
L Res1 R3
U 1 1 65AEE56F
P 3600 9600
F 0 "R3" V 3710 9470 60  0000 R TNN
F 1 "36K" V 3610 9470 60  0000 R TNN
F 2 "" H 3610 9470 60  0000 C CNN
F 3 "" H 3610 9470 60  0000 C CNN
	1    3600 9600
	0    -1   -1   0
$EndComp
$Comp
L Res1 R8
U 1 1 65AEE56E
P 4000 9200
F 0 "R8" H 3900 9100 60  0000 L BNN
F 1 "100" H 4100 9100 60  0000 L BNN
F 2 "" H 4100 9100 60  0000 C CNN
F 3 "" H 4100 9100 60  0000 C CNN
	1    4000 9200
	1    0    0    -1
$EndComp
$Comp
L STM32F103CBT6 U5
U 1 1 65AEE56D
P 2500 5200
F 0 "U5" H 4600 5100 60  0000 L BNN
F 1 "STM32F103CBT6" H 4600 5000 60  0000 L BNN
F 2 "" H 4600 5000 60  0000 C CNN
F 3 "" H 4600 5000 60  0000 C CNN
	1    2500 5200
	1    0    0    -1
$EndComp
$Comp
L Res1 R15
U 1 1 65AEE56C
P 6900 7600
F 0 "R15" V 6800 7480 60  0000 R TNN
F 1 "10K" V 6700 7480 60  0000 R TNN
F 2 "" H 6700 7480 60  0000 C CNN
F 3 "" H 6700 7480 60  0000 C CNN
	1    6900 7600
	0    -1    1    0
$EndComp
$Comp
L LD_BICOLOR_CMS LD1
U 1 1 65AEE56B
P 9700 5800
F 0 "LD1" H 10010 6100 60  0000 R BNN
F 1 "LD_BICOLOR_CMS" H 10010 5200 60  0000 R BNN
F 2 "" H 10010 5200 60  0000 C CNN
F 3 "" H 10010 5200 60  0000 C CNN
	1    9700 5800
	-1    0    0    -1
$EndComp
$Comp
L Res1 R7
U 1 1 65AEE56A
P 9700 8000
F 0 "R7" V 9600 7880 60  0000 R TNN
F 1 "2.7K" V 9500 7880 60  0000 R TNN
F 2 "" H 9500 7880 60  0000 C CNN
F 3 "" H 9500 7880 60  0000 C CNN
	1    9700 8000
	0    -1    1    0
$EndComp
$Comp
L CAP_SMD_0603__X5R_1µF_25V_10% C6
U 1 1 65AEE569
P 7400 7900
F 0 "C6" V 7510 7810 60  0000 R TNN
F 1 "4.7uF" V 7400 7800 60  0000 R TNN
F 2 "" H 7400 7800 60  0000 C CNN
F 3 "" H 7400 7800 60  0000 C CNN
	1    7400 7900
	0    -1   -1   0
$EndComp
$Comp
L Capacitor_ST C1
U 1 1 65AEE568
P 10000 7800
F 0 "C1" V 10010 7610 60  0000 R TNN
F 1 "100nF" V 9910 7610 60  0000 R TNN
F 2 "" H 9910 7610 60  0000 C CNN
F 3 "" H 9910 7610 60  0000 C CNN
	1    10000 7800
	0    -1   -1   0
$EndComp
$Comp
L Circuit_Breaker SB1
U 1 1 65AEE567
P 3600 4000
F 0 "SB1" H 3600 3800 60  0000 L BNN
	1    3600 4000
	1    0    0    -1
$EndComp
$Comp
L Header_3X2 CN2
U 1 1 65AEE566
P 9400 4700
F 0 "CN2" H 9400 4700 60  0000 L BNN
F 1 "[N/A]" H 9400 4200 60  0000 L BNN
F 2 "" H 9400 4200 60  0000 C CNN
F 3 "" H 9400 4200 60  0000 C CNN
	1    9400 4700
	1    0    0    -1
$EndComp
$Comp
L Circuit_Breaker SB9
U 1 1 65AEE565
P 8700 4000
F 0 "SB9" H 8700 3800 60  0000 L BNN
	1    8700 4000
	1    0    0    -1
$EndComp
$Comp
L ST890CDR U1
U 1 1 65AEE564
P 8400 7500
F 0 "U1" H 8400 7600 60  0000 L BNN
F 1 "ST890CDR" H 8400 7500 60  0000 L BNN
F 2 "" H 8400 7500 60  0000 C CNN
F 3 "" H 8400 7500 60  0000 C CNN
	1    8400 7500
	1    0    0    -1
$EndComp
$Comp
L 1050170001 CN1
U 1 1 65AEE563
P 1900 9500
F 0 "CN1" H 2700 9500 60  0000 R BNN
F 1 "1050170001" H 2700 8200 60  0000 R BNN
F 2 "" H 2700 8200 60  0000 C CNN
F 3 "" H 2700 8200 60  0000 C CNN
	1    1900 9500
	-1    0    0    -1
$EndComp
$Comp
L Res1 R1
U 1 1 65AEE562
P 2500 9500
F 0 "R1" H 2400 9400 60  0000 L BNN
F 1 "1K5" H 2700 9400 60  0000 L BNN
F 2 "" H 2700 9400 60  0000 C CNN
F 3 "" H 2700 9400 60  0000 C CNN
	1    2500 9500
	1    0    0    -1
$EndComp
$Comp
L Res1 R2
U 1 1 65AEE561
P 2500 9800
F 0 "R2" H 2400 9700 60  0000 L BNN
F 1 "100K" H 2700 9700 60  0000 L BNN
F 2 "" H 2700 9700 60  0000 C CNN
F 3 "" H 2700 9700 60  0000 C CNN
	1    2500 9800
	1    0    0    -1
$EndComp
$Comp
L Res1 R18
U 1 1 65AEE560
P 9000 5600
F 0 "R18" H 8990 5530 60  0000 L BNN
F 1 "100" H 8990 5370 60  0000 L BNN
F 2 "" H 8990 5370 60  0000 C CNN
F 3 "" H 8990 5370 60  0000 C CNN
	1    9000 5600
	1    0    0    -1
$EndComp
$Comp
L Res1 R19
U 1 1 65AEE55F
P 9000 6000
F 0 "R19" H 8990 5930 60  0000 L BNN
F 1 "100" H 8990 5770 60  0000 L BNN
F 2 "" H 8990 5770 60  0000 C CNN
F 3 "" H 8990 5770 60  0000 C CNN
	1    9000 6000
	1    0    0    -1
$EndComp
$Comp
L Res1 R17
U 1 1 65AEE55E
P 9900 6000
F 0 "R17" H 9890 5930 60  0000 L BNN
F 1 "0" H 10100 5800 60  0000 L BNN
F 2 "" H 10100 5800 60  0000 C CNN
F 3 "" H 10100 5800 60  0000 C CNN
	1    9900 6000
	1    0    0    -1
$EndComp
$EndSCHEMATC