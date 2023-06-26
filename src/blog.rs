use dioxus::prelude::*;

#[macro_export]
macro_rules! include_png {
    ($path:literal) => {
        format_args!(
            "data:image/png;base64,{}",
            base64::encode(include_bytes!($path))
        )
    };
}
pub fn App(cx: Scope) -> Element{
    cx.render(rsx!(
             style{ include_str!("blog.css") }
             div{
                class: "basis",
                div{
                    class: "topbar",
                    p{
                        class: "blogsite-name",
                        "Blogzy",
                       }
                    div{
                        class: "genres",
                        button{
                            class: "subsection",
                            "Sci-Fi"
                        }
                        button{
                            class: "subsection",
                            "Horror"
                        }
                        button{
                            class: "subsection",
                            "Psych"
                        }
                        button{
                            class: "subsection",
                            "Thriller"
                        }
                        button{
                            class: "subsection",
                            "Mystery"
                        }
                        button{
                            class: "subsection",
                            "Non-Fiction"
                        }
                        button{
                            class: "subsection",
                            "More"
                        }
                    }
                }
                 div{
                    class: "article-page",
                        div{
                            h1{
                                "To Hide or Not To Hide? Understanding Hamlet and Winstons Use of Facades."
                            }
                            img {
                                class: "head-img",
                                src: include_png!("images/masks.png")
                            }
                            p{
                                class: "article-details",
                                "Author: Fawzi, 
                                 Date: 4/?/1948"
                            }
                        }
                         div{
                                class: "main-body",
                                p{
                                    class: "subheader",
                                    "SPOILERS"
                                }
                                p{
                                    class: "paragraph",
                                    "This article requires some pre-requisit reading on 1984 and Hamlet. If you some how haven't read these, then go grab a copy from the local library
                                    and comeback soon!"
                                }
                                p{
                                    class: "subheader",
                                    "
                                    [Fun Introductory Subheader]
                                    "
                                }
                                p{
                                    class: "paragraph",
                                    "
                                    Hamlet and Winston are characters that both wear a facade. In Hamlet's case, he actively chooses to wear a facade to punish Claudius and Gertrude for betraying his Father.
                                    While in Winston's case, the facade is forced onto him even though he fights against it at the endangerment of himself. Using tools from psychoanalysis theory we can draw parallels between Winston and Hamlet use of a fake persona,
                                    why they would be inclined to use it, and when does an act become real. 
                                    "
                                }
                                p{
                                    class: "paragraph",
                                    "At the beginning of the play, Hamlet is denied to grieve his father’s death by the marriage of his uncle and mother. Hamlet views the marriage as incestuous and unholy showing that he maintains Christian values. After finding out Hamlet’s father was killed by his uncle, he strongly desires to break and kill him. He puts on an act and feigns insanity to hide his intent from his uncle, mother, and everyone around him. His desire for revenge causes him to lash out against his love interest and harms many other people around him. 
                                    In 1984, Winston buys a diary and writes down his thoughts even though it is against the party’s laws. He hates Big Brother and the party, but he protects himself with a facade so that no one may find out. During the two-minute hate, Winston joined in on yelling at Goldstein. He did this because he had to maintain his act, but also because he couldn’t control himself to not join the rest of his colleagues who were throwing books and stammering.
                                    "
                                }
                                div{
                                    class: "id-panel",
                                    div{
                                        class: "id-content",
                                        p{
                                            class: "subheader",
                                            "Id"
                                        }
                                        p{
                                            class: "paragraph",
                                            "
                                            Hamlet has a strong feeling of hate and a desire for revenge. Due to these feelings, he feigns insanity and drags everyone into his revenge. He harasses his significant other, murders her father, and betrays his values. When given the opportunity to kill his uncle during prayer, he opts to instead to kill him in a state that would guarantee he goes to hell. Hamlet hated his uncle for murdering his father and enjoying himself without any look of guilt, but when seeing his uncle praying, remorseful for his actions, he doesn’t kill him. His desire for revenge has so much control over him that he can’t just kill him even though he has what he wants.
                                            "
                                        }
                                        p{
                                            class: "paragraph",
                                            "	Winston desires to think and follow the emotions he feels completely free of the constraints of the party. He enters the restricted proletariate region and purchases a journal. He scribbles his thoughts down in the journal and reminisces about his day. He even scribbles down anti-government thoughts such as “down with big brother” filling up a page of his journal. Winston takes off his facade of being an indoctrinated and obedient citizen whenever he writes in the journal. Winston writes about his feelings of love and lust towards Julia which he expresses more openly to her as the book progresses. In the case of Winston, the Id rages against the party for shackling it. "
                                        }
                                    }
                                }
//                                p{
//                                    class: "quote",
//                                    "“If you want to keep a secret you must keep it from yourself.”",
//                                }
                                div{
                                    class: "ego-panel",
                                    div{
                                        p{
                                            class: "subheader",
                                            "Super-Ego"
                                        }
                                        p{
                                            class: "paragraph",
                                            "The Superego is the ethical segment of the subconscious that is developed by cultural practices. As a member of Denmark royalty, Hamlet was most likely raised as a Christian. It is considered to be incestuous if a brother marries his dead brother’s widow, which explains Hamlet’s despise for his mother. Hamlet even doubts his father’s ghost, thinking it might be the devil. His motivation for revenge was guided by some Christian values at first, such as punishing regicide and incest. When presented with the opportunity to kill his uncle, he refuses to ensure he is condemned to hell. This act of sacrilege shows how far the Id has grasped Hamlet and how the Superego begins to fall apart."
                                        }
                                        p{
                                            class: "paragraph",
                                            "The party’s influence on Winston through the use of propaganda and indoctrination builds the basis of his superego. When the two minute hate happened he finds himself chanting alongside the others, condemning Goldstein and celebrating Big Brother and the party, even though his Id hates them. While Winston becomes more open about himself he also becomes more paranoid, believing that the party will ultimately catch him. This fear is the superego punishing him for releasing his desires more and more."
                                        }
                                    }
                                }
                                div{
                                    class: "superego-panel",
                                    div{
                                        class: "superego-content",
                                        p{
                                            class: "subheader",
                                            "Ego"
                                        }
                                        p{
                                            class: "paragraph",
                                            "The ego is the mediator between the Id and Superego. Hamlet’s mind goes through multiple conflicts throughout the play. Hamlet seeks revenge, but also wishes to confirm that his uncle murdered his father. The ego’s answer to this conflict is to form a facade that both decives and tests his uncle. Another conflict is when he plans to go to his mother’s bed chamber. We see the ego rectify his id’s desire of killing his mother. He says his tongue and soul will be hypocrites and he will speak daggers to her but not use any. He soul wishes to kill his mother but the superego demands he doesn’t. The ego in this situation compromises by hurting her verbally.Hamlet’s ego tries to manage the conflict between the Id and superego but through the play, we see it side closer to the id."
                                        }
                                        p{
                                            class: "paragraph",
                                            "Winston’s ego manages the fear of the Superego and desires of the Id. The ego compromises many times by using a private journal for him to sate his desire of rebelling against the party while also keeping him secure from getting caught. As Winston desire grows stronger, his ego finds different ways of settling the conflict between Id and Superego, such as confiding in Julia instead of privately in his journal. As the story progresses, Winston desires for freedom and rebellion makes him want to be an enemy of the party. He stills fears the party and what they will do if they catch him, so the ego’s compromise is to join Goldstein’s Brotherhood where he can rebel safely with a community."
                                        }
                                    }
                                }
                         
                                div{
                                    p{
                                        class: "Bsubheader",
                                        "Brain Complete!"
                                    }
                                    div{
                                        class: "brain-icon",
                                        svg{
                                            fill:"#000000",
                                            height:"470px",
                                            width:"450px",
                                            version:"1.1",
                                            id:"Capa_1",
                                            xmlns:"http://www.w3.org/2000/svg",
                                           g{ 
                                                id:"SVGRepo_bgCarrier",
                                                stroke_width: "0",
                                            }
                                            g{
                                                id:"SVGRepo_tracerCarrier",
                                                stroke_linecap:"round",
                                                stroke_linejoin:"round",
                                            }
                                            g{
                                                id:"SVGRepo_iconCarrier"
                                            }
                                            path{
                                                d:"M151.245,222.446C148.054,237.039,135.036,248,119.5,248c-4.142,0-7.5,3.357-7.5,7.5s3.358,7.5,7.5,7.5 c23.774,0,43.522-17.557,46.966-40.386c14.556-1.574,27.993-8.06,38.395-18.677c2.899-2.959,2.85-7.708-0.109-10.606 c-2.958-2.897-7.707-2.851-10.606,0.108C184.947,202.829,172.643,208,159.5,208c-26.743,0-48.5-21.757-48.5-48.5 c0-4.143-3.358-7.5-7.5-7.5s-7.5,3.357-7.5,7.5C96,191.715,120.119,218.384,151.245,222.446z",
                                            }
                                            path{
                                                d:"M183,287.5c0-4.143-3.358-7.5-7.5-7.5c-35.014,0-63.5,28.486-63.5,63.5c0,0.362,0.013,0.725,0.019,1.088 C109.23,344.212,106.39,344,103.5,344c-4.142,0-7.5,3.357-7.5,7.5s3.358,7.5,7.5,7.5c26.743,0,48.5,21.757,48.5,48.5 c0,4.143,3.358,7.5,7.5,7.5s7.5-3.357,7.5-7.5c0-26.611-16.462-49.437-39.731-58.867c-0.178-1.699-0.269-3.418-0.269-5.133 c0-26.743,21.757-48.5,48.5-48.5C179.642,295,183,291.643,183,287.5z"
                                                }
                                            path{
                                                d:"M439,223.5c0-17.075-6.82-33.256-18.875-45.156c1.909-6.108,2.875-12.426,2.875-18.844 c0-30.874-22.152-56.659-51.394-62.329C373.841,91.6,375,85.628,375,79.5c0-19.557-11.883-36.387-28.806-43.661 C317.999,13.383,287.162,0,263.5,0c-13.153,0-24.817,6.468-32,16.384C224.317,6.468,212.653,0,199.5,0 c-23.662,0-54.499,13.383-82.694,35.839C99.883,43.113,88,59.943,88,79.5c0,6.128,1.159,12.1,3.394,17.671 C62.152,102.841,40,128.626,40,159.5c0,6.418,0.965,12.735,2.875,18.844C30.82,190.244,24,206.425,24,223.5 c0,13.348,4.149,25.741,11.213,35.975C27.872,270.087,24,282.466,24,295.5c0,23.088,12.587,44.242,32.516,55.396 C56.173,353.748,56,356.626,56,359.5c0,31.144,20.315,58.679,49.79,68.063C118.611,449.505,141.965,463,167.5,463 c27.995,0,52.269-16.181,64-39.674c11.731,23.493,36.005,39.674,64,39.674c25.535,0,48.889-13.495,61.71-35.437 c29.475-9.385,49.79-36.92,49.79-68.063c0-2.874-0.173-5.752-0.516-8.604C426.413,339.742,439,318.588,439,295.5 c0-13.034-3.872-25.413-11.213-36.025C434.851,249.241,439,236.848,439,223.5z M167.5,448c-21.029,0-40.191-11.594-50.009-30.256 c-0.973-1.849-2.671-3.208-4.688-3.751C88.19,407.369,71,384.961,71,359.5c0-3.81,0.384-7.626,1.141-11.344 c0.702-3.447-1.087-6.92-4.302-8.35C50.32,332.018,39,314.626,39,295.5c0-8.699,2.256-17.014,6.561-24.379 C56.757,280.992,71.436,287,87.5,287c4.142,0,7.5-3.357,7.5-7.5s-3.358-7.5-7.5-7.5C60.757,272,39,250.243,39,223.5 c0-14.396,6.352-27.964,17.428-37.221c2.5-2.09,3.365-5.555,2.14-8.574C56.2,171.869,55,165.744,55,159.5 c0-26.743,21.757-48.5,48.5-48.5s48.5,21.757,48.5,48.5c0,4.143,3.358,7.5,7.5,7.5s7.5-3.357,7.5-7.5 c0-33.642-26.302-61.243-59.421-63.355C104.577,91.127,103,85.421,103,79.5c0-13.369,8.116-24.875,19.678-29.859 c0.447-0.133,0.885-0.307,1.308-0.527C127.568,47.752,131.447,47,135.5,47c12.557,0,23.767,7.021,29.256,18.325 c1.81,3.727,6.298,5.281,10.023,3.47c3.726-1.809,5.28-6.296,3.47-10.022c-6.266-12.903-18.125-22.177-31.782-25.462 C165.609,21.631,184.454,15,199.5,15c13.509,0,24.5,10.99,24.5,24.5v97.051c-6.739-5.346-15.25-8.551-24.5-8.551 c-4.142,0-7.5,3.357-7.5,7.5s3.358,7.5,7.5,7.5c13.509,0,24.5,10.99,24.5,24.5v180.279c-9.325-12.031-22.471-21.111-37.935-25.266 c-3.999-1.071-8.114,1.297-9.189,5.297c-1.075,4.001,1.297,8.115,5.297,9.189C206.8,343.616,224,366.027,224,391.5 C224,422.654,198.654,448,167.5,448z M395.161,339.807c-3.215,1.43-5.004,4.902-4.302,8.35c0.757,3.718,1.141,7.534,1.141,11.344 c0,25.461-17.19,47.869-41.803,54.493c-2.017,0.543-3.716,1.902-4.688,3.751C335.691,436.406,316.529,448,295.5,448 c-31.154,0-56.5-25.346-56.5-56.5c0-2.109-0.098-4.2-0.281-6.271c0.178-0.641,0.281-1.314,0.281-2.012V135.5 c0-13.51,10.991-24.5,24.5-24.5c4.142,0,7.5-3.357,7.5-7.5s-3.358-7.5-7.5-7.5c-9.25,0-17.761,3.205-24.5,8.551V39.5 c0-13.51,10.991-24.5,24.5-24.5c15.046,0,33.891,6.631,53.033,18.311c-13.657,3.284-25.516,12.559-31.782,25.462 c-1.81,3.727-0.256,8.214,3.47,10.022c3.726,1.81,8.213,0.257,10.023-3.47C303.733,54.021,314.943,47,327.5,47 c4.053,0,7.933,0.752,11.514,2.114c0.422,0.22,0.86,0.393,1.305,0.526C351.883,54.624,360,66.13,360,79.5 c0,5.921-1.577,11.627-4.579,16.645C322.302,98.257,296,125.858,296,159.5c0,4.143,3.358,7.5,7.5,7.5s7.5-3.357,7.5-7.5 c0-26.743,21.757-48.5,48.5-48.5s48.5,21.757,48.5,48.5c0,6.244-1.2,12.369-3.567,18.205c-1.225,3.02-0.36,6.484,2.14,8.574 C417.648,195.536,424,209.104,424,223.5c0,26.743-21.757,48.5-48.5,48.5c-4.142,0-7.5,3.357-7.5,7.5s3.358,7.5,7.5,7.5 c16.064,0,30.743-6.008,41.939-15.879c4.306,7.365,6.561,15.68,6.561,24.379C424,314.626,412.68,332.018,395.161,339.807z"
                                                }
                                            path{
                                                d:"M359.5,240c-15.536,0-28.554-10.961-31.745-25.554C358.881,210.384,383,183.715,383,151.5c0-4.143-3.358-7.5-7.5-7.5 s-7.5,3.357-7.5,7.5c0,26.743-21.757,48.5-48.5,48.5c-13.143,0-25.447-5.171-34.646-14.561c-2.898-2.958-7.647-3.007-10.606-0.108 s-3.008,7.647-0.109,10.606c10.402,10.617,23.839,17.103,38.395,18.677C315.978,237.443,335.726,255,359.5,255 c4.142,0,7.5-3.357,7.5-7.5S363.642,240,359.5,240z"
                                                } 
                                            path 
                                            {
                                                d:"M335.5,328c-2.89,0-5.73,0.212-8.519,0.588c0.006-0.363,0.019-0.726,0.019-1.088c0-35.014-28.486-63.5-63.5-63.5 c-4.142,0-7.5,3.357-7.5,7.5s3.358,7.5,7.5,7.5c26.743,0,48.5,21.757,48.5,48.5c0,1.714-0.091,3.434-0.269,5.133 C288.462,342.063,272,364.889,272,391.5c0,4.143,3.358,7.5,7.5,7.5s7.5-3.357,7.5-7.5c0-26.743,21.757-48.5,48.5-48.5 c4.142,0,7.5-3.357,7.5-7.5S339.642,328,335.5,328z"
                                            }
                                        g{ }

                                        }    
                                    }
                                }
                                div{
                                    p{
                                        class: "subheader",
                                        "Conclusion"
                                    }
                                    p{
                                        class: "paragraph",
                                        "I think Hamlet’s facade more than just being a way to protect him is a manifestation of the conflict his ego deals with. The more and more the ego shifts to the side of id, the more and more his madness becomes real. Hamlet’s act inadvertently hurt too many people which led to his murder.  Winston wore his facade to protect himself from the laws of his society. As the story progresses, he comes to trust people like Julia and O’Brien and finds it easier to take off his facade of being an indoctrinated supporter of the party. Due to Winston removing his facade, he was betrayed, tortured, and killed. Both of them are the opposing sides of the same coin. Hamlet and Winston wear their facades as a solution to the internal conflict inside them. In the case of Hamlet the Id forces him to become mad, while Winston's Id forces him to embrace freedom from the his act."
                                    }
                                }
                        }
                }
            }   
        ))
}