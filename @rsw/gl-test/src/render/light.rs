use nalgebra::Vector3;

pub trait Light<T> {
    fn color(&self) -> Vector3<T>;
    fn intensity(&self) -> T;
}

pub trait LightPosition<T> {
    fn position(&self) -> Vector3<T>;
}

pub trait LightDirection<T> {
    fn direction(&self) -> Vector3<T>;
}

pub type AmbientLight = AmbientLightGen<f32>;

pub struct AmbientLightGen<T> {
    pub color: Vector3<T>,
    pub intensity: T,
}

impl <T> Light<T> for AmbientLightGen<T> 
where T: Copy {
    fn color(&self) -> Vector3<T> {
        self.color
    }

    fn intensity(&self) -> T {
        self.intensity
    }
}

pub type DirectionalLight = DirectionalLightGen<f32>;

pub struct DirectionalLightGen<T> {
    pub color: Vector3<T>,
    pub intensity: T,
    pub direction: Vector3<T>,
}

impl <T> Light<T> for DirectionalLightGen<T>
where T: Copy {
    fn color(&self) -> Vector3<T> {
        self.color
    }

    fn intensity(&self) -> T {
        self.intensity
    }
}

impl <T> LightDirection<T> for DirectionalLightGen<T>
where T: Copy {
    fn direction(&self) -> Vector3<T> {
        self.direction
    }
}

pub type PointLight = PointLightGen<f32>;

pub struct PointLightGen<T> {
    pub color: Vector3<T>,
    pub intensity: T,
    pub position: Vector3<T>,
}

impl <T> Light<T> for PointLightGen<T>
where T: Copy {
    fn color(&self) -> Vector3<T> {
        self.color
    }

    fn intensity(&self) -> T {
        self.intensity
    }
}

impl <T> LightPosition<T> for PointLightGen<T>
where T: Copy {
    fn position(&self) -> Vector3<T> {
        self.position
    }
}

pub type SpotLight = SpotLightGen<f32>;

pub struct SpotLightGen<T> {
    pub color: Vector3<T>,
    pub intensity: T,
    pub position: Vector3<T>,
    pub direction: Vector3<T>,
}

impl <T> Light<T> for SpotLightGen<T>
where T: Copy {
    fn color(&self) -> Vector3<T> {
        self.color
    }

    fn intensity(&self) -> T {
        self.intensity
    }
}

impl <T> LightPosition<T> for SpotLightGen<T>
where T: Copy {
    fn position(&self) -> Vector3<T> {
        self.position
    }
}

impl <T> LightDirection<T> for SpotLightGen<T>
where T: Copy {
    fn direction(&self) -> Vector3<T> {
        self.direction
    }
}
