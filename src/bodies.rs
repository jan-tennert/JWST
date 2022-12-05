use bevy::prelude::Vec3;

use crate::body::BodyBundle;

const AU_TO_UNIT_SCALE: f32 = 10.0;

pub struct Body {
    pub model: String,
    pub body: BodyBundle,
    pub radius: f32,
    pub model_scale: f32,
    pub name: String,
    pub unlit: bool
}

impl Body {
    
    pub fn earth() -> Self {
        Body {
            model: "models/earth.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                5.97219,
                Vec3::new(
                    4.487758087146768E-01, 8.751235324844499E-01, 1.618817013329493E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    -1.552868871220300E-02, 7.906229533085379E-03, 3.064648367334892E-07
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Earth".to_string(),
            unlit: false
        }
    }
    
    pub fn saturn() -> Self {
        Body {
            model: "models/saturn.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                568.3,
                Vec3::new(
                    8.032503665636328E+00, -5.674419409731062E+00, -2.211472254846864E-01
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    2.906095271828988E-03, 4.545286691593917E-03, -1.944528757086951E-04
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Saturn".to_string(),
            unlit: false
        }
    }
    
    pub fn jupiter() -> Self {
        Body {
            model: "models/jupiter.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                1898.187,
                Vec3::new(
                  4.883310383356100E+00, 7.577598574024473E-01, -1.123963322175233E-01
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   -1.243645389952230E-03, 7.811788737744427E-03, -4.555620902846121E-06
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Jupiter".to_string(),
            unlit: false
        }
    }
    
    pub fn mars() -> Self {
        Body {
            model: "models/mars.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                0.64171,
                Vec3::new(
                  5.371347489929870E-01, 1.415777733841128E+00, 1.647268731293564E-02
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   -1.252424659948937E-02, 6.220232033014156E-03, 4.378447959849454E-04
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Mars".to_string(),
            unlit: false
        }
    }
    
    pub fn uranus() -> Self {
        Body {
            model: "models/uranus.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                86.813,
                Vec3::new(
                  1.346817163779143E+01, 1.433467548071632E+01, -1.212433131756314E-01
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   -2.895294668494246E-03, 2.509923332168401E-03, 4.682362547589839E-05
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Uranus".to_string(),
            unlit: false
        }
    }
    
    pub fn mercury() -> Self {
        Body {
            model: "models/mercury.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                0.3302,
                Vec3::new(
                  3.111405698823826E-03, -4.607974584403516E-01, -3.860159093638146E-02
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   2.248220104774973E-02, 2.177869427789603E-03, -1.883369041847100E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Mercury".to_string(),
            unlit: false
        }
    }
    
    pub fn venus() -> Self {
        Body {
            model: "models/venus.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                4.867,
                Vec3::new(
                    -1.104602952742054E-01, -7.189512888891817E-01, -3.818331146763080E-03
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    1.989263354357960E-02, -2.910502841093310E-03, -1.187594747165436E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Venus".to_string(),
            unlit: false
        }
    }
    
    pub fn pluto() -> Self {
        Body {
            model: "models/pluto.glb#Scene0".to_string(),
            radius: 0.005,
            body: BodyBundle::new(
                0.0130900,
                Vec3::new(
                  1.606202476106402E+01, -3.066989614373318E+01, -1.364243998730049E+00
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    2.848861795045802E-03, 7.648276574228828E-04, -9.055284692410262E-04
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Pluto".to_string(),
            unlit: false
        }
    }
    
    pub fn moon() -> Self {
        Body {
            model: "models/moon.glb#Scene0".to_string(),
            radius: 0.002,
            body: BodyBundle::new(
                0.0734767,
                Vec3::new(
                4.482115265952957E-01, 8.727621196450731E-01, 3.888179917645140E-05
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.491883668334010E-02, 7.773993419863166E-03, -4.679176055656679E-05
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.000003,
            name: "Moon".to_string(),
            unlit: false
        }
    }
    
    pub fn jwst() -> Self {
        Body {
            model: "models/jwst.glb#Scene0".to_string(),
            radius: 0.002,
            body: BodyBundle::new(
                6200.0e-24,
                Vec3::new(
                4.488948840878112E-01, 8.856860339483225E-01, -7.512566561474845E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.564139006806661E-02, 7.940335006606503E-03, -9.026475694712961E-05
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.0003,
            name: "JWST".to_string(),
            unlit: true
        }
    }
    
    pub fn iss() -> Self {
        Body {
            model: "models/iss.glb#Scene0".to_string(),
            radius: 0.002,
            body: BodyBundle::new(
                0.000000000000000000000444615,
                Vec3::new(
                4.488043238527515E-01, 8.751376110417752E-01, 1.941969204321329E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.504701637582184E-02, 1.177161600016825E-02, -2.108766983760775E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.000003,
            name: "ISS".to_string(),
            unlit: false
        }
    }
    
    pub fn hubble() -> Self {
        Body {
            model: "models/hubble.glb#Scene0".to_string(),
            radius: 0.002,
            body: BodyBundle::new(
                11600.0 / f32::powf(10.0, 24.0),
                Vec3::new(
                4.487378270154649E-01, 8.751063507720495E-01, 1.817069672476064E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.303624904379408E-02, 5.738929759261992E-03, 2.893730048309336E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.000003,
            name: "Hubble".to_string(),
            unlit: false
        }
    }
    
}