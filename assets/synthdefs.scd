SynthDef.new("pluck", {|amp=1,freq=440,gate=1,out=0,pan=0,
        susT=1,
        fmod=0, blur=1|
    var osc, env;
    susT = susT * blur;
    freq = [freq, freq+fmod];
    amp=(amp + 1e-05) * 0.8;
    freq=(freq + [0, LFNoise2.ar(50).range(-2, 2)]);
    osc=((SinOsc.ar((freq * 1.002), phase: VarSaw.ar(freq, width: Line.ar(1, 0.2, 2))) * 0.3) + (SinOsc.ar(freq, phase: VarSaw.ar(freq, width: Line.ar(1, 0.2, 2))) * 0.3));
    osc=((osc * XLine.kr(amp, (amp / 10000), (susT * 4), doneAction: Done.freeSelf)) * 0.3);
    osc = Mix(osc) * 0.5;
    osc = Pan2.ar(osc, pan);
    Out.ar(out, osc)})

