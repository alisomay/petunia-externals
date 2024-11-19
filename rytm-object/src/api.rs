pub mod global;
pub mod kit;
pub mod pattern;
pub mod settings;
pub mod sound;

pub mod plock;

use crate::value::RytmValue;

// TODO: Sub module error types insert

/*** Object Types ***/

pub mod object_type {

    pub const PATTERN: &str = "pattern";
    pub const PATTERN_WORK_BUFFER: &str = "pattern_wb";
    pub const KIT: &str = "kit";
    pub const KIT_WORK_BUFFER: &str = "kit_wb";
    pub const SOUND: &str = "sound";
    pub const SOUND_WORK_BUFFER: &str = "sound_wb";
    pub const GLOBAL: &str = "global";
    pub const GLOBAL_WORK_BUFFER: &str = "global_wb";
    pub const SETTINGS: &str = "settings";

    pub const OBJECT_TYPES: &[&str] = &[
        PATTERN,
        PATTERN_WORK_BUFFER,
        KIT,
        KIT_WORK_BUFFER,
        SOUND,
        SOUND_WORK_BUFFER,
        GLOBAL,
        GLOBAL_WORK_BUFFER,
        SETTINGS,
    ];
}

/*** Object Element Types ***/

pub mod kit_element_type {
    pub const TRACK_LEVEL: &str = "tracklevel";
    pub const TRACK_RETRIG_RATE: &str = "trackretrigrate";
    pub const TRACK_RETRIG_LENGTH: &str = "trackretriglen";
    pub const TRACK_RETRIG_VEL_OFFSET: &str = "trackretrigveloffset";
    pub const TRACK_RETRIG_ALWAYS_ON: &str = "trackretrigalwayson";
    pub const SOUND: &str = "sound";

    pub const KIT_ELEMENTS: &[&str] = &[
        TRACK_LEVEL,
        TRACK_RETRIG_RATE,
        TRACK_RETRIG_LENGTH,
        TRACK_RETRIG_VEL_OFFSET,
        TRACK_RETRIG_ALWAYS_ON,
        SOUND,
    ];

    pub const KIT_ELEMENTS_ACTION: &[&str] =
        &[TRACK_LEVEL, TRACK_RETRIG_VEL_OFFSET, TRACK_RETRIG_ALWAYS_ON];

    pub const KIT_ELEMENTS_ENUM: &[&str] = &[TRACK_RETRIG_RATE, TRACK_RETRIG_LENGTH];
}

pub mod plock_type {
    pub const PLOCK_GET: &str = "plockget";
    pub const PLOCK_SET: &str = "plockset";
    pub const PLOCK_CLEAR: &str = "plockclear";

    pub const PLOCK_TYPES: &[&str] = &[PLOCK_GET, PLOCK_SET, PLOCK_CLEAR];
}

// TODO: MACHINE
pub mod machine_parameter_type {
    // TODO: For the first version we'll omit machine parameters.
}

/*** Action Types ***/

pub mod settings_action_type {
    pub const VERSION: &str = "version";
    pub const BPM_PROJECT: &str = "projectbpm";
    pub const SELECTED_TRACK: &str = "selectedtrack";
    pub const SELECTED_PAGE: &str = "selectedpage";
    pub const MUTE: &str = "mute";
    pub const UNMUTE: &str = "unmute";
    pub const FIXED_VELOCITY_ENABLE: &str = "fixedvelocity";
    pub const FIXED_VELOCITY_AMOUNT: &str = "fixedvelocityamt";
    pub const SAMPLE_RECORDER_THR: &str = "samplerecorderthr";
    pub const SAMPLE_RECORDER_MONITOR_ENABLE: &str = "samplerecordermon";

    pub const SETTINGS_ACTION_TYPES: &[&str] = &[
        VERSION,
        BPM_PROJECT,
        SELECTED_TRACK,
        SELECTED_PAGE,
        MUTE,
        UNMUTE,
        FIXED_VELOCITY_ENABLE,
        FIXED_VELOCITY_AMOUNT,
        SAMPLE_RECORDER_THR,
        SAMPLE_RECORDER_MONITOR_ENABLE,
    ];
}

pub mod global_action_type {
    pub const VERSION: &str = "version";
    pub const INDEX: &str = "index";
    pub const IS_WORK_BUFFER: &str = "iswb";

    pub const KIT_RELOAD_ON_CHANGE: &str = "kitreloadonchg";
    pub const QUANTIZE_LIVE_REC: &str = "quantizeliverec";
    pub const AUTO_TRACK_SWITCH: &str = "autotrackswitch";

    pub const ROUTE_TO_MAIN: &str = "routetomain";
    pub const SEND_TO_FX: &str = "sendtofx";

    pub const CLOCK_RECEIVE: &str = "clockreceive";
    pub const CLOCK_SEND: &str = "clocksend";
    pub const TRANSPORT_RECEIVE: &str = "transportreceive";
    pub const TRANSPORT_SEND: &str = "transportsend";
    pub const PROGRAM_CHANGE_RECEIVE: &str = "pgmchangereceive";
    pub const PROGRAM_CHANGE_SEND: &str = "pgmchangesend";

    pub const RECEIVE_NOTES: &str = "receivenotes";
    pub const RECEIVE_CC_NRPN: &str = "receiveccnrpn";
    // Only get will be implemented for this one
    pub const TURBO_SPEED: &str = "turbospeed";

    pub const METRONOME_ACTIVE: &str = "metronomeactive";
    pub const METRONOME_PRE_ROLL_BARS: &str = "metronomeprerollbars";
    pub const METRONOME_VOLUME: &str = "metronomelev";

    pub const GLOBAL_ACTION_TYPES: &[&str] = &[
        VERSION,
        INDEX,
        IS_WORK_BUFFER,
        KIT_RELOAD_ON_CHANGE,
        QUANTIZE_LIVE_REC,
        AUTO_TRACK_SWITCH,
        ROUTE_TO_MAIN,
        SEND_TO_FX,
        CLOCK_RECEIVE,
        CLOCK_SEND,
        TRANSPORT_RECEIVE,
        TRANSPORT_SEND,
        PROGRAM_CHANGE_RECEIVE,
        PROGRAM_CHANGE_SEND,
        RECEIVE_NOTES,
        RECEIVE_CC_NRPN,
        TURBO_SPEED,
        METRONOME_ACTIVE,
        METRONOME_PRE_ROLL_BARS,
        METRONOME_VOLUME,
    ];
}

pub mod kit_action_type {
    pub const VERSION: &str = "version";
    pub const INDEX: &str = "index";
    pub const NAME: &str = "name";

    pub const CONTROL_IN_1_MOD_AMT: &str = "ctrlinmod1amt";
    pub const CONTROL_IN_2_MOD_AMT: &str = "ctrlinmod2amt";

    pub const FX_DELAY_TIME: &str = "fxdeltime";
    pub const FX_DELAY_PING_PONG: &str = "fxdelpingpong";
    pub const FX_DELAY_STEREO_WIDTH: &str = "fxdelstereowidth";
    pub const FX_DELAY_FEEDBACK: &str = "fxdelfeedback";
    pub const FX_DELAY_HPF: &str = "fxdelhpf";
    pub const FX_DELAY_LPF: &str = "fxdellpf";
    pub const FX_DELAY_REVERB_SEND: &str = "fxdelrevsend";
    pub const FX_DELAY_VOLUME: &str = "fxdellev";

    pub const FX_REVERB_PRE_DELAY: &str = "fxrevpredel";
    pub const FX_REVERB_DECAY: &str = "fxrevdecay";
    pub const FX_REVERB_FREQ: &str = "fxrevfreq";
    pub const FX_REVERB_GAIN: &str = "fxrevgain";
    pub const FX_REVERB_HPF: &str = "fxrevhpf";
    pub const FX_REVERB_LPF: &str = "fxrevlpf";
    pub const FX_REVERB_VOLUME: &str = "fxrevlev";

    pub const FX_COMP_THRESHOLD: &str = "fxcompthr";
    pub const FX_COMP_GAIN: &str = "fxcompgain";
    pub const FX_COMP_MIX: &str = "fxcompmix";
    pub const FX_COMP_VOLUME: &str = "fxcomplev";

    pub const FX_LFO_SPEED: &str = "fxlfospeed";
    pub const FX_LFO_FADE: &str = "fxlfofade";
    pub const FX_LFO_START_PHASE_OR_SLEW: &str = "fxlfostartphase";
    pub const FX_LFO_DEPTH: &str = "fxlfodepth";

    pub const FX_DISTORTION_DELAY_OVERDRIVE: &str = "fxdistdov";
    pub const FX_DISTORTION_DELAY_POST: &str = "fxdistdelpost";
    pub const FX_DISTORTION_REVERB_POST: &str = "fxdistrevpost";
    pub const FX_DISTORTION_AMOUNT: &str = "fxdistamt";
    pub const FX_DISTORTION_SYMMETRY: &str = "fxdistsym";

    pub const KIT_ACTION_TYPES: &[&str] = &[
        VERSION,
        INDEX,
        NAME,
        CONTROL_IN_1_MOD_AMT,
        CONTROL_IN_2_MOD_AMT,
        FX_DELAY_TIME,
        FX_DELAY_PING_PONG,
        FX_DELAY_STEREO_WIDTH,
        FX_DELAY_FEEDBACK,
        FX_DELAY_HPF,
        FX_DELAY_LPF,
        FX_DELAY_REVERB_SEND,
        FX_DELAY_VOLUME,
        FX_REVERB_PRE_DELAY,
        FX_REVERB_DECAY,
        FX_REVERB_FREQ,
        FX_REVERB_GAIN,
        FX_REVERB_HPF,
        FX_REVERB_LPF,
        FX_REVERB_VOLUME,
        FX_COMP_THRESHOLD,
        FX_COMP_GAIN,
        FX_COMP_MIX,
        FX_COMP_VOLUME,
        FX_LFO_SPEED,
        FX_LFO_FADE,
        FX_LFO_START_PHASE_OR_SLEW,
        FX_LFO_DEPTH,
        FX_DISTORTION_DELAY_OVERDRIVE,
        FX_DISTORTION_DELAY_POST,
        FX_DISTORTION_REVERB_POST,
        FX_DISTORTION_AMOUNT,
        FX_DISTORTION_SYMMETRY,
    ];
}

pub mod trig_action_type {

    pub const ENABLE: &str = "enable";
    pub const RETRIG: &str = "retrig";
    pub const MUTE: &str = "mute";
    pub const ACCENT: &str = "accent";
    pub const SWING: &str = "swing";
    pub const SLIDE: &str = "slide";

    // TODO: I need to understand how these behave first.
    // Also maybe we expose them in the parameter lock set action?
    // pub const PARAMETER_LOCK_LFO_SWITCH: &str = "parameterlocklfoswitch";
    // pub const PARAMETER_LOCK_LFO: &str = "parameterlocklfo";
    // pub const PARAMETER_LOCK_SYNTH_SWITCH: &str = "parameterlocksynthswitch";
    // pub const PARAMETER_LOCK_SYNTH: &str = "parameterlocksynth";
    // pub const PARAMETER_LOCK_SAMPLE_SWITCH: &str = "parameterlocksampleswitch";
    // pub const PARAMETER_LOCK_SAMPLE: &str = "parameterlocksample";
    // pub const PARAMETER_LOCK_ENV_SWITCH: &str = "parameterlockenvswitch";
    // pub const PARAMETER_LOCK_ENV: &str = "parameterlockenv";

    pub const NOTE: &str = "note";
    pub const VELOCITY: &str = "vel";
    pub const RETRIG_VELOCITY_OFFSET: &str = "retrigveloffset";
    pub const SOUND_LOCK: &str = "soundlock";

    pub const TRIG_ACTION_TYPES: &[&str] = &[
        ENABLE,
        RETRIG,
        MUTE,
        ACCENT,
        SWING,
        SLIDE,
        // PARAMETER_LOCK_LFO_SWITCH
        // PARAMETER_LOCK_LFO
        // PARAMETER_LOCK_SYNTH_SWITCH
        // PARAMETER_LOCK_SYNTH
        // PARAMETER_LOCK_SAMPLE_SWITCH
        // PARAMETER_LOCK_SAMPLE
        // PARAMETER_LOCK_ENV_SWITCH
        // PARAMETER_LOCK_ENV
        NOTE,
        VELOCITY,
        RETRIG_VELOCITY_OFFSET,
        SOUND_LOCK,
    ];
}

pub mod track_action_type {
    pub const IS_WORK_BUFFER: &str = "iswb";
    pub const OWNER_INDEX: &str = "parentindex";
    pub const INDEX: &str = "index";
    pub const DEF_TRIG_NOTE: &str = "deftrignote";
    pub const DEF_TRIG_VELOCITY: &str = "deftrigvel";
    pub const DEF_TRIG_PROB: &str = "deftrigprob";
    pub const NUMBER_OF_STEPS: &str = "steps";
    pub const QUANTIZE_AMOUNT: &str = "quantizeamount";
    pub const SENDS_MIDI: &str = "sendsmidi";
    // TODO: Revise ranges
    pub const EUCLIDEAN_MODE: &str = "euc";
    pub const EUCLIDEAN_PL1: &str = "pl1";
    pub const EUCLIDEAN_PL2: &str = "pl2";
    pub const EUCLIDEAN_RO1: &str = "ro1";
    pub const EUCLIDEAN_RO2: &str = "ro2";
    pub const EUCLIDEAN_TRO: &str = "tro";

    pub const TRACK_ACTION_TYPES: &[&str] = &[
        IS_WORK_BUFFER,
        OWNER_INDEX,
        INDEX,
        DEF_TRIG_NOTE,
        DEF_TRIG_VELOCITY,
        DEF_TRIG_PROB,
        NUMBER_OF_STEPS,
        QUANTIZE_AMOUNT,
        SENDS_MIDI,
        EUCLIDEAN_MODE,
        EUCLIDEAN_PL1,
        EUCLIDEAN_PL2,
        EUCLIDEAN_RO1,
        EUCLIDEAN_RO2,
        EUCLIDEAN_TRO,
    ];
}

pub mod pattern_action_type {
    pub const IS_WORK_BUFFER: &str = "iswb";
    pub const INDEX: &str = "index";
    pub const VERSION: &str = "version";
    pub const MASTER_LENGTH: &str = "masterlen";
    pub const MASTER_CHANGE: &str = "masterchg";
    pub const KIT_NUMBER: &str = "kitnumber";
    pub const SWING_AMOUNT: &str = "swingamount";
    pub const GLOBAL_QUANTIZE: &str = "globalquantize";
    pub const BPM: &str = "patternbpm";

    // TODO: Newly found settings
    // pub const PAD_SCALE_PER_TRACK: &str = "padscalepertrack";

    pub const PATTERN_ACTION_TYPES: &[&str] = &[
        IS_WORK_BUFFER,
        INDEX,
        VERSION,
        MASTER_LENGTH,
        MASTER_CHANGE,
        KIT_NUMBER,
        SWING_AMOUNT,
        GLOBAL_QUANTIZE,
        BPM,
        // PAD_SCALE_PER_TRACK
    ];
}

pub mod sound_action_type {
    pub const VERSION: &str = "version";
    pub const INDEX: &str = "index";
    pub const NAME: &str = "name";

    pub const IS_POOL: &str = "ispool";
    pub const IS_KIT: &str = "iskit";
    pub const IS_WORK_BUFFER: &str = "iswb";

    pub const KIT_NUMBER: &str = "kitnumber";
    pub const SOUND_TYPE: &str = "type";
    pub const ACCENT_LEVEL: &str = "accentlev";

    pub const AMP_ATTACK: &str = "ampattack";
    pub const AMP_HOLD: &str = "amphold";
    pub const AMP_DECAY: &str = "ampdecay";
    pub const AMP_OVERDRIVE: &str = "ampoverdrive";
    pub const AMP_DELAY_SEND: &str = "ampdelsend";
    pub const AMP_REVERB_SEND: &str = "amprevsend";
    pub const AMP_PAN: &str = "amppan";
    pub const AMP_VOLUME: &str = "amplev";

    pub const FILT_ATTACK: &str = "filtattack";
    pub const FILT_HOLD: &str = "filthold";
    pub const FILT_DECAY: &str = "filtdecay";
    pub const FILT_RELEASE: &str = "filtrelease";
    pub const FILT_CUTOFF: &str = "filtcutoff";
    pub const FILT_RESONANCE: &str = "filtres";
    pub const FILT_ENVELOPE_AMOUNT: &str = "filtenvamt";

    pub const LFO_SPEED: &str = "lfospeed";
    pub const LFO_FADE: &str = "lfofade";
    pub const LFO_START_PHASE_OR_SLEW: &str = "lfostartphase";
    pub const LFO_DEPTH: &str = "lfodepth";

    pub const SAMP_TUNE: &str = "samptune";
    pub const SAMP_FINE_TUNE: &str = "sampfinetune";
    pub const SAMP_NUMBER: &str = "sampnumber";
    pub const SAMP_BIT_REDUCTION: &str = "sampbitreduction";
    pub const SAMP_START: &str = "sampstart";
    pub const SAMP_END: &str = "sampend";
    pub const SAMP_LOOP_FLAG: &str = "samploopflag";
    pub const SAMP_VOLUME: &str = "samplev";

    pub const VEL_MOD_AMT: &str = "velmodamt";
    pub const AT_MOD_AMT: &str = "atmodamt";

    pub const ENV_RESET_FILTER: &str = "envresetfilter";
    pub const VELOCITY_TO_VOLUME: &str = "veltovol";
    pub const LEGACY_FX_SEND: &str = "legacyfxsend";

    pub const SOUND_ACTION_TYPES: &[&str] = &[
        VERSION,
        INDEX,
        NAME,
        IS_POOL,
        IS_KIT,
        IS_WORK_BUFFER,
        KIT_NUMBER,
        SOUND_TYPE,
        ACCENT_LEVEL,
        AMP_ATTACK,
        AMP_HOLD,
        AMP_DECAY,
        AMP_OVERDRIVE,
        AMP_DELAY_SEND,
        AMP_REVERB_SEND,
        AMP_PAN,
        AMP_VOLUME,
        FILT_ATTACK,
        FILT_HOLD,
        FILT_DECAY,
        FILT_RELEASE,
        FILT_CUTOFF,
        FILT_RESONANCE,
        FILT_ENVELOPE_AMOUNT,
        LFO_SPEED,
        LFO_FADE,
        LFO_START_PHASE_OR_SLEW,
        LFO_DEPTH,
        SAMP_TUNE,
        SAMP_FINE_TUNE,
        SAMP_NUMBER,
        SAMP_BIT_REDUCTION,
        SAMP_START,
        SAMP_END,
        SAMP_LOOP_FLAG,
        SAMP_VOLUME,
        VEL_MOD_AMT,
        AT_MOD_AMT,
        ENV_RESET_FILTER,
        VELOCITY_TO_VOLUME,
        LEGACY_FX_SEND,
    ];
}

/*** Enum Types ***/

pub mod pattern_enum_type {
    pub const SPEED: &str = "speed";
    pub const TIME_MODE: &str = "timemode";

    pub const PATTERN_ENUM_TYPES: &[&str] = &[SPEED, TIME_MODE];
}

pub mod track_enum_type {
    pub const ROOT_NOTE: &str = "rootnote";
    pub const PAD_SCALE: &str = "padscale";
    pub const DEFAULT_NOTE_LENGTH: &str = "defaultnotelen";

    pub const TRACK_ENUM_TYPES: &[&str] = &[ROOT_NOTE, PAD_SCALE, DEFAULT_NOTE_LENGTH];
}

pub mod trig_enum_type {
    pub const MICRO_TIME: &str = "microtime";
    pub const NOTE_LENGTH: &str = "notelen";
    pub const RETRIG_LENGTH: &str = "retriglen";
    pub const RETRIG_RATE: &str = "retrigrate";
    pub const TRIG_CONDITION: &str = "trigcondition";

    pub const TRIG_ENUM_TYPES: &[&str] = &[
        MICRO_TIME,
        NOTE_LENGTH,
        RETRIG_LENGTH,
        RETRIG_RATE,
        TRIG_CONDITION,
    ];
}

pub mod kit_enum_type {
    pub const CONTROL_IN_1_MOD_TARGET: &str = "ctrlinmod1target";
    pub const CONTROL_IN_2_MOD_TARGET: &str = "ctrlinmod2target";

    pub const FX_LFO_DESTINATION: &str = "fxlfodest";
    // Only set, when getting you can get it with FX_DELAY_TIME
    pub const FX_DELAY_TIME_ON_THE_GRID: &str = "fxdeltimeonthegrid";
    pub const FX_COMP_ATTACK: &str = "fxcompattack";
    pub const FX_COMP_RELEASE: &str = "fxcomprelease";
    pub const FX_COMP_RATIO: &str = "fxcompratio";
    pub const FX_COMP_SIDE_CHAIN_EQ: &str = "fxcompsidechaineq";

    pub const KIT_ENUM_TYPES: &[&str] = &[
        CONTROL_IN_1_MOD_TARGET,
        CONTROL_IN_2_MOD_TARGET,
        FX_LFO_DESTINATION,
        FX_DELAY_TIME_ON_THE_GRID,
        FX_COMP_ATTACK,
        FX_COMP_RELEASE,
        FX_COMP_RATIO,
        FX_COMP_SIDE_CHAIN_EQ,
    ];
}

pub mod settings_enum_type {
    pub const PARAMETER_MENU_ITEM: &str = "parametermenuitem";
    pub const FX_PARAMETER_MENU_ITEM: &str = "fxparametermenuitem";
    pub const SEQUENCER_MODE: &str = "sequencermode";
    pub const PATTERN_MODE: &str = "patternmode";
    pub const SAMPLE_RECORDER_SOURCE: &str = "samplerecordersrc";
    pub const SAMPLE_RECORDER_RECORDING_LENGTH: &str = "samplerecorderrecordinglen";

    pub const SETTINGS_ENUM_TYPES: &[&str] = &[
        PARAMETER_MENU_ITEM,
        FX_PARAMETER_MENU_ITEM,
        SEQUENCER_MODE,
        PATTERN_MODE,
        SAMPLE_RECORDER_SOURCE,
        SAMPLE_RECORDER_RECORDING_LENGTH,
    ];
}

pub mod sound_enum_type {
    pub const MACHINE_PARAMETERS: &str = "machineparameters";
    pub const MACHINE_TYPE: &str = "machinetype";
    pub const LFO_DESTINATION: &str = "lfodest";
    pub const VELOCITY_MOD_TARGET: &str = "velmodtarget";
    pub const AFTER_TOUCH_MOD_TARGET: &str = "atmodtarget";
    pub const FILTER_TYPE: &str = "filtertype";
    pub const LFO_MULTIPLIER: &str = "lfomultiplier";
    pub const LFO_WAVEFORM: &str = "lfowaveform";
    pub const LFO_MODE: &str = "lfomode";
    pub const SOUND_SETTINGS_CHROMATIC_MODE: &str = "chromaticmode";

    pub const SOUND_ENUM_TYPES: &[&str] = &[
        MACHINE_PARAMETERS,
        MACHINE_TYPE,
        LFO_DESTINATION,
        VELOCITY_MOD_TARGET,
        AFTER_TOUCH_MOD_TARGET,
        FILTER_TYPE,
        LFO_MULTIPLIER,
        LFO_WAVEFORM,
        LFO_MODE,
        SOUND_SETTINGS_CHROMATIC_MODE,
    ];
}

pub mod sound_machine_enum_type {
    pub const BD_ACOUSTIC_WAVEFORM: &str = "bdacousticwaveform";
    pub const BD_SHARP_WAVEFORM: &str = "bdsharpwaveform";
    pub const SY_CHIP_WAVEFORM: &str = "sychipwaveform";
    pub const SY_CHIP_SPEED: &str = "sychipspeed";
    pub const SY_RAW_WAVEFORM_1: &str = "syrawwaveform1";
    pub const SY_RAW_WAVEFORM_2: &str = "syrawwaveform2";

    pub const SOUND_MACHINE_ENUM_TYPES: &[&str] = &[
        BD_ACOUSTIC_WAVEFORM,
        BD_SHARP_WAVEFORM,
        SY_CHIP_WAVEFORM,
        SY_CHIP_SPEED,
        SY_RAW_WAVEFORM_1,
        SY_RAW_WAVEFORM_2,
    ];
}

pub mod global_enum_type {
    pub const METRONOME_TIME_SIGNATURE: &str = "metronometimesig";

    pub const ROUTING_USB_IN_OPTIONS: &str = "usbin";
    pub const ROUTING_USB_OUT_OPTIONS: &str = "usbout";
    pub const ROUTING_USB_TO_MAIN_DB: &str = "usbtomaindb";

    pub const OUT_PORT_FUNCTION: &str = "outportfunction";
    pub const THRU_PORT_FUNCTION: &str = "thruportfunction";
    pub const INPUT_FROM: &str = "inputfrom";
    pub const OUTPUT_TO: &str = "outputto";
    pub const PARAM_OUTPUT: &str = "paramoutput";
    pub const PAD_DEST: &str = "paddest";
    pub const PRESSURE_DEST: &str = "pressuredest";
    pub const ENCODER_DEST: &str = "encoderdest";
    pub const MUTE_DEST: &str = "mutedest";
    pub const PORTS_OUTPUT_CHANNEL: &str = "portsoutputchannel";

    pub const AUTO_CHANNEL: &str = "autochannel";
    pub const TRACK_CHANNELS: &str = "trackchannels";
    pub const TRACK_FX_CHANNEL: &str = "trackfxchannel";
    pub const PROGRAM_CHANGE_IN_CHANNEL: &str = "pgmchangeinchannel";
    pub const PROGRAM_CHANGE_OUT_CHANNEL: &str = "pgmchangeoutchannel";
    pub const PERFORMANCE_CHANNEL: &str = "performancechannel";

    pub const GLOBAL_ENUM_TYPES: &[&str] = &[
        METRONOME_TIME_SIGNATURE,
        ROUTING_USB_IN_OPTIONS,
        ROUTING_USB_OUT_OPTIONS,
        ROUTING_USB_TO_MAIN_DB,
        OUT_PORT_FUNCTION,
        THRU_PORT_FUNCTION,
        INPUT_FROM,
        OUTPUT_TO,
        PARAM_OUTPUT,
        PAD_DEST,
        PRESSURE_DEST,
        ENCODER_DEST,
        MUTE_DEST,
        PORTS_OUTPUT_CHANNEL,
        AUTO_CHANNEL,
        TRACK_CHANNELS,
        TRACK_FX_CHANNEL,
        PROGRAM_CHANGE_IN_CHANNEL,
        PROGRAM_CHANGE_OUT_CHANNEL,
        PERFORMANCE_CHANNEL,
    ];
}

#[derive(Debug)]
pub enum Response {
    Common {
        index: usize,
        key: RytmValue,
        value: RytmValue,
    },
    KitElement {
        kit_index: usize,
        element_index: usize,
        element_type: RytmValue,
        value: RytmValue,
    },
    Track {
        pattern_index: usize,
        track_index: usize,
        key: RytmValue,
        value: RytmValue,
    },
    Trig {
        pattern_index: usize,
        track_index: usize,
        trig_index: usize,
        key: RytmValue,
        value: RytmValue,
    },
    Ok,
}
