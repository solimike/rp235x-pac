#[doc = "Register `CTIOUTEN2` reader"]
pub type R = crate::R<CTIOUTEN2_SPEC>;
#[doc = "Register `CTIOUTEN2` writer"]
pub type W = crate::W<CTIOUTEN2_SPEC>;
#[doc = "Field `TRIGOUTEN` reader - Enables a cross trigger event to ctitrigout when the corresponding channel is activated. There is one bit of the field for each of the four channels."]
pub type TRIGOUTEN_R = crate::FieldReader;
#[doc = "Field `TRIGOUTEN` writer - Enables a cross trigger event to ctitrigout when the corresponding channel is activated. There is one bit of the field for each of the four channels."]
pub type TRIGOUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Enables a cross trigger event to ctitrigout when the corresponding channel is activated. There is one bit of the field for each of the four channels."]
    #[inline(always)]
    pub fn trigouten(&self) -> TRIGOUTEN_R {
        TRIGOUTEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enables a cross trigger event to ctitrigout when the corresponding channel is activated. There is one bit of the field for each of the four channels."]
    #[inline(always)]
    #[must_use]
    pub fn trigouten(&mut self) -> TRIGOUTEN_W<CTIOUTEN2_SPEC> {
        TRIGOUTEN_W::new(self, 0)
    }
}
#[doc = "CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIOUTEN2_SPEC;
impl crate::RegisterSpec for CTIOUTEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiouten2::R`](R) reader structure"]
impl crate::Readable for CTIOUTEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctiouten2::W`](W) writer structure"]
impl crate::Writable for CTIOUTEN2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIOUTEN2 to value 0"]
impl crate::Resettable for CTIOUTEN2_SPEC {
    const RESET_VALUE: u32 = 0;
}
