#[cfg(feature = "quirks")]
#[macro_export]
macro_rules! vf_reset {
    ($self:expr) => {
        if $self.quirks.vf_reset {
            $self.regs[0xf] = 0;
        }
    };
}

#[cfg(not(feature = "quirks"))]
#[macro_export]
macro_rules! vf_reset {
    ($self:expr) => {
        $self.regs[0xf] = 0;
    };
}

#[cfg(feature = "quirks")]
#[macro_export]
macro_rules! memory {
    ($self:expr) => {
        if $self.quirks.memory {
            $self.i = $self.i.saturating_add(1);
        }
    };
}

#[cfg(not(feature = "quirks"))]
#[macro_export]
macro_rules! memory {
    ($self:expr) => {
        $self.i = $self.i.saturating_add(1);
    };
}

#[cfg(feature = "quirks")]
#[macro_export]
macro_rules! display_blank {
    ( $self:expr ) => {
        if $self.quirks.display_blank && $self.wait_vblank != WaitVblank::Vblank {
            $self.pause_vblank();
            return;
        }
        $self.wait_vblank = WaitVblank::NotWaiting;
    };
}

#[cfg(not(feature = "quirks"))]
#[macro_export]
macro_rules! display_blank {
    ( $self:expr ) => {};
}

#[cfg(feature = "quirks")]
#[macro_export]
macro_rules! clipping {
    ( $self:expr, $y:expr, $y0:expr, $yf:expr ) => {
        if $self.quirks.clipping {
            $yf = $y0 + $y;
            if $yf >= DISPLAY_HEIGHT {
                continue;
            }
        } else {
            $yf = ($y0 + $y) % DISPLAY_HEIGHT;
        }
    };
}

#[cfg(not(feature = "quirks"))]
#[macro_export]
macro_rules! clipping {
    ( $self:expr, $y:expr, $y0:expr, $yf:expr ) => {
        $yf = $y0 + $y;
        if $yf >= DISPLAY_HEIGHT {
            continue;
        }
    }
}

#[cfg(feature = "quirks")]
#[macro_export]
macro_rules! shifting {
    ( $self:expr, $x:expr, $y:expr, $shift:tt ) => {
        if $self.quirks.shifting {
            $self.regs[$x] = $self.regs[$x] $shift 1;
        } else {
            $self.regs[$x] = $self.regs[$y] $shift 1;
        }
    }
}

#[cfg(not(feature = "quirks"))]
#[macro_export]
macro_rules! shifting {
    ( $self:expr, $x:expr, $y:expr, $shift:tt ) => {
        $self.regs[$x] = $self.regs[$y] $shift 1;
    }
}

#[cfg(feature = "quirks")]
#[macro_export]
macro_rules! jumping {
    ( $self:expr, $address:expr, $x:expr ) => {
        if $self.quirks.jumping {
            $self.pc = $address + $self.regs[$x] as u16;
        } else {
            $self.pc = $address + $self.regs[0x0] as u16;
        }
    };
}

#[cfg(not(feature = "quirks"))]
#[macro_export]
macro_rules! jumping {
    ( $self:expr, $address:expr, $x:expr ) => {
        $self.pc = $address + $self.regs[0x0] as u16;
    };
}
