use crate::ping::{
    CborPingSchema, IpV4Wrapper, IpV6Wrapper, IpWrapper, JobEntry, JobType, MAX_REQ_SZ, PING_JOB,
    PJob, Stats,
};
use heapless::Vec;
use minicbor::{
    Decode, Decoder, Encode, Encoder,
    decode::ArrayIterWithCtx,
    encode::{Error, Write},
};

impl<C> Encode<C> for JobEntry {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.u64(self.id)?.encode(self.job.clone())?.ok()
    }
}

impl<C> Encode<C> for PJob {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.str_len(PING_JOB.len() as u64)?.str(PING_JOB)?.ok()?;

        for (ip, stat) in &self.target {
            e.map(self.target.len() as u64)?.ok()?;
            e.encode(ip)?.ok()?;
            e.encode(stat)?.ok()?;
        }
        Ok(())
    }
}

impl<C> Encode<C> for IpV6Wrapper {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.0.octets())?.ok()
    }
}

impl<C> Encode<C> for IpV4Wrapper {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _: &mut C) -> Result<(), Error<W::Error>> {
        e.bytes(&self.0.octets())?.ok()
    }
}

impl<C> Encode<C> for IpWrapper {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.array(2)?;
        match self {
            IpWrapper::V4(ip) => e.u32(0)?.encode_with(ip, ctx)?.ok(),
            IpWrapper::V6(ip) => e.u32(1)?.encode_with(ip, ctx)?.ok(),
        }
    }
}

impl<'b, C> Decode<'b, C> for IpV4Wrapper {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let octets: minicbor::bytes::ByteArray<4> = Decode::decode(d, ctx)?;
        Ok(IpV4Wrapper(<[u8; 4]>::from(octets).into()))
    }
}

impl<'b, C> Decode<'b, C> for IpV6Wrapper {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let octets: minicbor::bytes::ByteArray<16> = Decode::decode(d, ctx)?;
        Ok(IpV6Wrapper(<[u8; 16]>::from(octets).into()))
    }
}

impl<'b, C> Decode<'b, C> for IpWrapper {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let p = d.position();
        if Some(2) != d.array()? {
            return Err(minicbor::decode::Error::message("expected enum (2-element array)").at(p));
        }
        let p = d.position();
        match d.i64()? {
            0 => Ok(IpWrapper::V4(IpV4Wrapper::decode(d, ctx)?)),
            1 => Ok(IpWrapper::V6(IpV6Wrapper::decode(d, ctx)?)),
            n => Err(minicbor::decode::Error::unknown_variant(n).at(p)),
        }
    }
}

impl<'b, C> Decode<'b, C> for CborPingSchema {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let _ = d.decode::<&str>()?;
        let kype = match d.decode::<u8>()? {
            0 => JobType::Ping,
            1 => JobType::PortScan,
            2 => JobType::Broadcast,
            _ => return Err(minicbor::decode::Error::type_mismatch(d.datatype()?)),
        };

        let arr: ArrayIterWithCtx<'_, '_, C, (IpWrapper, Stats)> = d.array_iter_with(ctx)?;
        let mut ip_list: Vec<(IpWrapper, Stats), MAX_REQ_SZ> = Vec::new();
        for i in arr {
            ip_list
                .push(i?)
                .map_err(|_| minicbor::decode::Error::message("Vector push failed"))?;
        }
        let count = d.decode::<Option<u32>>()?;
        Ok(CborPingSchema {
            kype,
            ip_list,
            count,
        })
    }
}
