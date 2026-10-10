use leptos::prelude::*;

const BAMBOO_HEIGHT: f64 = 1400.0;
const SEGMENT_LENGTH: f64 = 120.0;
const NODE_GAP: f64 = 5.0;
const LEAF_PATH: &str = "M0 0 C12 -7 30 -7 46 0 C30 5 12 5 0 0Z";

struct LeafSpray {
    height: f64,
    angles: &'static [f64],
}

struct Stalk {
    x: f64,
    width: f64,
    top: f64,
    lean: f64,
    sprays: &'static [LeafSpray],
}

const LEFT_STALKS: [Stalk; 3] = [
    Stalk {
        x: 12.0,
        width: 12.0,
        top: -40.0,
        lean: -1.5,
        sprays: &[
            LeafSpray {
                height: 1040.0,
                angles: &[-25.0, 5.0, 30.0],
            },
            LeafSpray {
                height: 520.0,
                angles: &[-15.0, 20.0],
            },
        ],
    },
    Stalk {
        x: 54.0,
        width: 10.0,
        top: 260.0,
        lean: 2.5,
        sprays: &[
            LeafSpray {
                height: 280.0,
                angles: &[200.0, 165.0, 140.0],
            },
            LeafSpray {
                height: 760.0,
                angles: &[-20.0, 15.0],
            },
        ],
    },
    Stalk {
        x: 92.0,
        width: 8.0,
        top: 640.0,
        lean: 4.0,
        sprays: &[LeafSpray {
            height: 640.0,
            angles: &[200.0, 170.0, -30.0],
        }],
    },
];

const RIGHT_STALKS: [Stalk; 2] = [
    Stalk {
        x: 16.0,
        width: 11.0,
        top: 100.0,
        lean: -2.0,
        sprays: &[
            LeafSpray {
                height: 900.0,
                angles: &[-30.0, 0.0, 28.0],
            },
            LeafSpray {
                height: 400.0,
                angles: &[195.0, 160.0],
            },
        ],
    },
    Stalk {
        x: 62.0,
        width: 9.0,
        top: 480.0,
        lean: 3.0,
        sprays: &[LeafSpray {
            height: 600.0,
            angles: &[-20.0, 12.0, 40.0],
        }],
    },
];

/// Decorative bamboo and a panda drawn behind the game. The panda is awake in the light
/// theme and asleep in the dark theme, which style.css switches between.
#[component]
pub fn Scenery() -> impl IntoView {
    view! {
        <div class="scenery" aria-hidden="true">
            <BambooCluster stalks=&LEFT_STALKS class="bamboo left" />
            <BambooCluster stalks=&RIGHT_STALKS class="bamboo right" />
            <Panda />
        </div>
    }
}

#[component]
fn BambooCluster(stalks: &'static [Stalk], class: &'static str) -> impl IntoView {
    view! {
        <svg class=class viewBox=format!("0 0 160 {BAMBOO_HEIGHT}") preserveAspectRatio="xMinYMax slice">
            {stalks.iter().map(|stalk| view! { <BambooStalk stalk /> }).collect_view()}
        </svg>
    }
}

#[component]
fn BambooStalk(stalk: &'static Stalk) -> impl IntoView {
    let center = stalk.x + stalk.width / 2.0;
    let segment_tops = (1..)
        .map(|index| BAMBOO_HEIGHT - index as f64 * SEGMENT_LENGTH)
        .take_while(|segment_top| segment_top + SEGMENT_LENGTH > stalk.top);

    view! {
        <g transform=format!("rotate({} {center} {BAMBOO_HEIGHT})", stalk.lean)>
            {segment_tops
                .map(|segment_top| {
                    let top = segment_top.max(stalk.top);
                    view! {
                        <rect
                            class="bamboo-stalk"
                            x=stalk.x
                            y=top + NODE_GAP
                            width=stalk.width
                            height=segment_top + SEGMENT_LENGTH - top - NODE_GAP
                            rx=stalk.width / 2.5
                        />
                        <rect
                            class="bamboo-node"
                            x=stalk.x - 1.5
                            y=segment_top + SEGMENT_LENGTH - NODE_GAP
                            width=stalk.width + 3.0
                            height=NODE_GAP
                            rx=NODE_GAP / 2.0
                        />
                    }
                })
                .collect_view()}
            {stalk
                .sprays
                .iter()
                .flat_map(|spray| {
                    spray
                        .angles
                        .iter()
                        .map(move |angle| {
                            let transform = format!(
                                "translate({center} {}) rotate({angle})",
                                BAMBOO_HEIGHT - spray.height,
                            );
                            view! { <path class="bamboo-leaf" d=LEAF_PATH transform=transform /> }
                        })
                })
                .collect_view()}
        </g>
    }
}

#[component]
fn Panda() -> impl IntoView {
    view! {
        <svg class="panda" viewBox="0 0 140 130">
            <g class="panda-awake">
                <circle class="panda-patch" cx="36" cy="58" r="13" />
                <circle class="panda-patch" cx="104" cy="58" r="13" />
                <circle class="panda-fur" cx="70" cy="100" r="46" />
                <ellipse class="panda-patch" cx="53" cy="94" rx="10" ry="12.5" transform="rotate(22 53 94)" />
                <ellipse class="panda-patch" cx="87" cy="94" rx="10" ry="12.5" transform="rotate(-22 87 94)" />
                <circle class="panda-eye" cx="54" cy="92" r="3.5" />
                <circle class="panda-eye" cx="86" cy="92" r="3.5" />
                <ellipse class="panda-patch" cx="70" cy="110" rx="5" ry="3.5" />
                <ellipse class="panda-patch" cx="34" cy="128" rx="17" ry="10" />
                <ellipse class="panda-patch" cx="106" cy="128" rx="17" ry="10" />
            </g>
            <g class="panda-asleep">
                <circle class="panda-patch" cx="28" cy="74" r="8" />
                <circle class="panda-patch" cx="56" cy="72" r="8" />
                <path class="panda-sketch" d="M40 122 Q44 88 86 88 Q130 88 130 112 Q130 122 120 122 Z" />
                <circle class="panda-sketch" cx="42" cy="98" r="24" />
                <ellipse class="panda-patch" cx="34" cy="98" rx="6.5" ry="5.5" />
                <ellipse class="panda-patch" cx="51" cy="98" rx="6.5" ry="5.5" />
                <path class="panda-line" d="M30 98 Q34 101 38 98" />
                <path class="panda-line" d="M47 98 Q51 101 55 98" />
                <ellipse class="panda-patch" cx="42" cy="108" rx="3.5" ry="2.5" />
                <ellipse class="panda-patch" cx="60" cy="122" rx="13" ry="5.5" />
                <ellipse class="panda-patch" cx="116" cy="122" rx="12" ry="5.5" />
                <path class="panda-line" d="M84 40 h7 l-7 7 h7" />
                <path class="panda-line" d="M98 26 h9 l-9 9 h9" />
                <path class="panda-line" d="M114 10 h11 l-11 11 h11" />
            </g>
        </svg>
    }
}
