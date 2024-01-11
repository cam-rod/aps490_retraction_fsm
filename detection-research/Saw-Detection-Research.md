# Saw Detection Research

Bit of a brain dump, but hopefully it's understandable. Most important stuff under [The Math](#the-math).

## Research Review

**Math Review:**

$$
\begin{align*}
\rho &= \frac{R\cdot A}{\mathcal{L}} \\
s &= \rho^{-1} = \frac{L}{R \cdot A}
\end{align*}
$$

Many measurements use $S/m$, so the calculation is based on area instead of length. Makes much more sense.

> [!NOTE] Inter-Subject Variability of Skull Conductivity and Thickness in Calibrated Realistic Head Models
> 
> [doi:10.1016/j.neuroimage.2020.117353](https://doi.org/10.1016/j.neuroimage.2020.117353)

- Main focus was on improving EEGs for variation in skull conductivity. All subjects were right-handed.
- Measurements taken via EEG and MEG. Noted that conductivity falls off after 20 ms, so they can find the central point at that time to record residual variance (RV) comapared to a standard model.
- Base on calibrated level was 12.5 mS/m compacta, 45 mS/m spongiosa, RV (minimum of curve) was mean 8.54%, SD 3.44%, minimum 4%. Massive compacta ranging from 2.6 mS/m to 16.9 mS/m

![](Pasted%20image%2020240110235219.png)

![](Pasted%20image%2020240110235251.png)

| No | Correlation of Skull Conductivity to Thickness of... | $\rho$ (corr. val) | 95% CI (bootstrapped conf. int.) | $P$ |
| ---- | ---- | ---- | ---- | ---- |
| 1 | Outer-compacta | -0.25 | -0.59    0.17 | 0.210 |
| 2 | Spongiosa | 0.13 | -0.41    0.62 | 0.216 |
| 3 | Inner-compacta | 0.26 | -0.13    0.61 | 0.210 |

TL;DR

- Statistically significant relationship between overall skull thickness and conductivity ($\rho = 0.52,\,  CI = [0.19\: 0.75],\, P = 0.01$)
- no statistically significant relationship, just mildly weak correlation on a) increasing conductivity with spongiosa & inner compacta, and b) negatively with outer spongiosa

Notes:

- Only degree of freedom was skull conductivity. Stuff like compacta:spongiosa ratio was fixed, there may be additional interactions with CSF, dura mater, skin (doesn't apply in our case), etc.
- Mentions at the end (PMID: [16012669](https://pubmed.ncbi.nlm.nih.gov/16012669/)) which quoted 330 mS/m for the grey matter
    - The original measures resistivity, so it quoted $3\, \ohm \cdot m$ ($330\, mS/m$) grey matter and $7\, \ohm \cdot m$ ($140\, mS/m$) white matter

> [!NOTE] Dielectric Properties of Human Brain Tissue Measured Less Than 10 h Postmortem at Frequencies From 800 to 2450 MHz
> 
> [doi:10.1002/bem.10123](https://doi.org/10.1002/bem.10123)

> The question of possible postmortem changes of the dielectric properties of brain tissue in the frequency range above 100 MHz was investigated by Kraszewski et al. [1982] on cat tissue (100 MHz – 8 GHz) and by Burdette  et  al.  [1986]  on  canine  brain  at  2450  GHz.Contrary  to  Kraszewski  et  al.  [1982],  who  did  notfind significant differences in the dielectric properties above  100  MHz  before  and  a  few  hours  after  death, Burdette et al. [1986] reported a post mortal decrease of permittivity (~9%), as well as of conductivity (~18%),within  the  first  hour  after  death.

- Similar results repeated on 800-1900 MHz in 2003 (15%/11% conductivity, just 3-4% permittivity)

> [!NOTE]- Quick SawStop® recap
> 
> > The effect of the arrangement shown in _FIG. 2_ is to form two capacitors in series through the blade, creating a capaci tive shunt at the junction between the capacitors. Thus, the input signal is transmitted from charge plate **62** through the blade and onto charge plate **66**. As illustrated, exemplary contact detection system **32** also includes a second electrical system **88** connected to charge plate **66**, and configured to detect changes in the input signal received at charge plate **66**. When a user touches the saw blade, the capacitance of the user's body creates a capacitive load on the blade. As a result, the size of the capacitive shunt between the charge plates and the blade is increased, thereby reducing the charge that reaches plate 66. Thus, the magnitude of the input signal passed through the blade to plate **66** decreases when a user touches the blade. As will be discussed in more detail below, second electrical system **88** is configured to respond to this change in the input signal with an output signal to brake system 34.
> 
> i.e. The saw blade is a shunt to "ground" :)

SawStop used a 100kHz square sine wave at 12 V, plates between 1/32" and 1/2" (0.7-12.7 mm) away... way below these measurements and seemingly not comparable. From Wikipedia on permittivity:

> Various microwave measurement techniques are outlined in Chen _et al._.[[21]](https://en.wikipedia.org/wiki/Permittivity#cite_note-Chen-21) Typical errors for the [Hakki–Coleman method](https://en.wikipedia.org/w/index.php?title=Hakki%E2%80%93Coleman_method&action=edit&redlink=1 "Hakki–Coleman method (page does not exist)") employing a puck of material between conducting planes are about 0.3%.[[22]](https://en.wikipedia.org/wiki/Permittivity#cite_note-Sebastian-22)
> 
> - Low-frequency [time domain](https://en.wikipedia.org/wiki/Time_domain "Time domain") measurements (10⁻⁶ to 10³ Hz)
> - Low-frequency [frequency domain](https://en.wikipedia.org/wiki/Frequency_domain "Frequency domain") measurements (10⁻⁵ to 10⁶ Hz)
> - Reflective coaxial methods (10⁶ to 10¹⁰ Hz)
> - Transmission coaxial method (10⁸ to 10¹¹ Hz)

Luckily it references the classical source:

> [!NOTE] The dielectric properties of biological tissues: II. Measurements in the frequency range 10 Hz to 20 GHz
> 
> [doi:10.1088/0031-9155/41/11/002](https://doi.org/10.1088/0031-9155/41/11/002)

![](Pasted%20image%2020240111002310.png)

From a cow - around 95-90 mS/m in the 50-100 kHz range. Again, not too far off given the much lower frequencies of EEG/MEG, so these values can be treated as reliable.

![](Pasted%20image%2020240111002347.png)

Pig and sheep - looks to be around 30 mS/m at 300 kHz and 1 MHz. Given the other study was done at sub-1 kHz range (closer to 200 Hz), this seems reasonable.

![](Pasted%20image%2020240111002559.png)

For comparison of humans to other mammals (this is body fat btw) - tends to be well related, the tongue is even better

So based on the above confirmation:

- brain permittivity at 100 kHz is roughly $k = 3000$, conductivity 95 mS/m - below link suggests $k=12000$ and 465 mS/m, for reference, so 
- skull permittivity at over 1 kF/m hopefully conductivity 30 mS/m - confirmed in doi:10.1007/s00221-007-1258-8 to be around $k = 8000$ (so like $k=2000$ probably reasonable adjustment)

### The Math

![](Pasted%20image%2020240111023703.png)

Assumptions:

- 5 mm plate-to-blade gap
- 10 × 15 mm square plates
- BD104 saw blade (64 mm diameter, .25" (6.35 mm) thickness, 420 stainless) to permit mounting on upper side

Capacitance of pure air is 132 fF? Basically nothing, so any signals sent should be read almost purely. Dieletric breakdown of air is in the range of kV/mm, so no issues there.

If we add our blade, that capacitance on each side would increase by multiples (much larger area, distance cut in half). For simplicity, I assume roughly double the effective area, at half the distance, increaseing to 528 fF. Most metals have almost no permittivity, so the plate is effectively transparent to signals from the plates.

When the blade comes into contact with the brain, the much higher permittivity creates a capacitive load that shunts the signal away from the other plate, which can be detected as a missing signal (example of a cheap component that could generate this signal: [AD9833](https://www.analog.com/en/products/ad9833.html#product-overview). More details provided in US patent 7055417).

Considering the numbers provided:

- Brain ($k=2000$): effective 1 nF capacitance
- Skull ($k = 8000$): effective 4 nF capacitance

Capacitance would be tricky to measure, but luckily we can bypass that with the signal measurement. As previously noted, the skull is quite a bit less conductive than the skull, so we can look for changes in the signal at each point (skull contact and brain contact).

Testing needed to validate:

- A similar sized blade to observe the impact the unconnected blade has on the signal
- Electronic:
    - Signal generator like AD9833 and measurement using an ADC (also pretty cheap)
    - Small metal plates for our "air gap capacitor"
    - Microcontroller to observe changes and control response
    - LEDs for status, button for reset, switch for disable (see [FSM](https://drive.google.com/file/d/1Py5mPOEzWGCt8Khahn3qPYJQuFaksQNb/view?usp=sharing) for additional detail)
    - Test rig?
        - Mammal bone to observe impact of bone contact on signal (unironically a chicken bone might work well. As long as there sufficient data on that particular bone, we can extrapolate to the impact of the skull)
        - Some object with similar conductivity and permittivity to the brain

