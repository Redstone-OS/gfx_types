//! # Buffer Handle
//!
//! Handle opaco para buffers gerenciados pelo kernel.
//
/// Este handle é usado para referenciar buffers de display sem expor
/// detalhes de implementação.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct BufferHandle(pub u64);

impl BufferHandle {
    /// Handle inválido/nulo.
    pub const INVALID: Self = Self(0);

    /// Cria handle a partir de valor raw.
    #[inline]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Verifica se o handle é válido.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }

    /// Verifica se o handle é inválido.
    #[inline]
    pub const fn is_invalid(&self) -> bool {
        self.0 == 0
    }

    /// Retorna o valor bruto.
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Alias para as_u64().
    #[inline]
    pub const fn raw(&self) -> u64 {
        self.0
    }

    /// Retorna o valor como usize.
    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Extrai ID do buffer (lower 32 bits).
    #[inline]
    pub const fn id(&self) -> u32 {
        self.0 as u32
    }

    /// Extrai geração/versão (upper 32 bits).
    #[inline]
    pub const fn generation(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Cria um novo handle a partir de ID e geração.
    #[inline]
    pub const fn new(id: u32, generation: u32) -> Self {
        Self::from_id_gen(id, generation)
    }

    /// Cria handle a partir de ID e geração.
    #[inline]
    pub const fn from_id_gen(id: u32, generation: u32) -> Self {
        Self(((generation as u64) << 32) | (id as u64))
    }
}

impl From<u64> for BufferHandle {
    #[inline]
    fn from(raw: u64) -> Self {
        Self(raw)
    }
}

impl From<BufferHandle> for u64 {
    #[inline]
    fn from(h: BufferHandle) -> Self {
        h.0
    }
}
