/* automatically generated by rust-bindgen 0.66.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_mode_t = crate::ctypes::c_ushort;
pub type __kernel_ipc_pid_t = crate::ctypes::c_ushort;
pub type __kernel_uid_t = crate::ctypes::c_ushort;
pub type __kernel_gid_t = crate::ctypes::c_ushort;
pub type __kernel_old_dev_t = crate::ctypes::c_ushort;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_size_t = crate::ctypes::c_uint;
pub type __kernel_ssize_t = crate::ctypes::c_int;
pub type __kernel_ptrdiff_t = crate::ctypes::c_int;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
pub type mtd_info_t = mtd_info_user;
pub type erase_info_t = erase_info_user;
pub type region_info_t = region_info_user;
pub type nand_oobinfo_t = nand_oobinfo;
pub type nand_ecclayout_t = nand_ecclayout_user;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct erase_info_user {
pub start: __u32,
pub length: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct erase_info_user64 {
pub start: __u64,
pub length: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_oob_buf {
pub start: __u32,
pub length: __u32,
pub ptr: *mut crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_oob_buf64 {
pub start: __u64,
pub pad: __u32,
pub length: __u32,
pub usr_ptr: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_write_req {
pub start: __u64,
pub len: __u64,
pub ooblen: __u64,
pub usr_data: __u64,
pub usr_oob: __u64,
pub mode: __u8,
pub padding: [__u8; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_read_req_ecc_stats {
pub uncorrectable_errors: __u32,
pub corrected_bitflips: __u32,
pub max_bitflips: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_read_req {
pub start: __u64,
pub len: __u64,
pub ooblen: __u64,
pub usr_data: __u64,
pub usr_oob: __u64,
pub mode: __u8,
pub padding: [__u8; 7usize],
pub ecc_stats: mtd_read_req_ecc_stats,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_info_user {
pub type_: __u8,
pub flags: __u32,
pub size: __u32,
pub erasesize: __u32,
pub writesize: __u32,
pub oobsize: __u32,
pub padding: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct region_info_user {
pub offset: __u32,
pub erasesize: __u32,
pub numblocks: __u32,
pub regionindex: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct otp_info {
pub start: __u32,
pub length: __u32,
pub locked: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nand_oobinfo {
pub useecc: __u32,
pub eccbytes: __u32,
pub oobfree: [[__u32; 2usize]; 8usize],
pub eccpos: [__u32; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nand_oobfree {
pub offset: __u32,
pub length: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nand_ecclayout_user {
pub eccbytes: __u32,
pub eccpos: [__u32; 64usize],
pub oobavail: __u32,
pub oobfree: [nand_oobfree; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mtd_ecc_stats {
pub corrected: __u32,
pub failed: __u32,
pub badblocks: __u32,
pub bbtblocks: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_attach_req {
pub ubi_num: __s32,
pub mtd_num: __s32,
pub vid_hdr_offset: __s32,
pub max_beb_per1024: __s16,
pub disable_fm: __s8,
pub padding: [__s8; 9usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_mkvol_req {
pub vol_id: __s32,
pub alignment: __s32,
pub bytes: __s64,
pub vol_type: __s8,
pub flags: __u8,
pub name_len: __s16,
pub padding2: [__s8; 4usize],
pub name: [crate::ctypes::c_char; 128usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_rsvol_req {
pub bytes: __s64,
pub vol_id: __s32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_rnvol_req {
pub count: __s32,
pub padding1: [__s8; 12usize],
pub ents: [ubi_rnvol_req__bindgen_ty_1; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_rnvol_req__bindgen_ty_1 {
pub vol_id: __s32,
pub name_len: __s16,
pub padding2: [__s8; 2usize],
pub name: [crate::ctypes::c_char; 128usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_leb_change_req {
pub lnum: __s32,
pub bytes: __s32,
pub dtype: __s8,
pub padding: [__s8; 7usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_map_req {
pub lnum: __s32,
pub dtype: __s8,
pub padding: [__s8; 3usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_set_vol_prop_req {
pub property: __u8,
pub padding: [__u8; 7usize],
pub value: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ubi_blkcreate_req {
pub padding: [__s8; 128usize],
}
pub const MTD_ABSENT: u32 = 0;
pub const MTD_RAM: u32 = 1;
pub const MTD_ROM: u32 = 2;
pub const MTD_NORFLASH: u32 = 3;
pub const MTD_NANDFLASH: u32 = 4;
pub const MTD_DATAFLASH: u32 = 6;
pub const MTD_UBIVOLUME: u32 = 7;
pub const MTD_MLCNANDFLASH: u32 = 8;
pub const MTD_WRITEABLE: u32 = 1024;
pub const MTD_BIT_WRITEABLE: u32 = 2048;
pub const MTD_NO_ERASE: u32 = 4096;
pub const MTD_POWERUP_LOCK: u32 = 8192;
pub const MTD_SLC_ON_MLC_EMULATION: u32 = 16384;
pub const MTD_CAP_ROM: u32 = 0;
pub const MTD_CAP_RAM: u32 = 7168;
pub const MTD_CAP_NORFLASH: u32 = 3072;
pub const MTD_CAP_NANDFLASH: u32 = 1024;
pub const MTD_CAP_NVRAM: u32 = 7168;
pub const MTD_NANDECC_OFF: u32 = 0;
pub const MTD_NANDECC_PLACE: u32 = 1;
pub const MTD_NANDECC_AUTOPLACE: u32 = 2;
pub const MTD_NANDECC_PLACEONLY: u32 = 3;
pub const MTD_NANDECC_AUTOPL_USR: u32 = 4;
pub const MTD_OTP_OFF: u32 = 0;
pub const MTD_OTP_FACTORY: u32 = 1;
pub const MTD_OTP_USER: u32 = 2;
pub const MTD_MAX_OOBFREE_ENTRIES: u32 = 8;
pub const MTD_MAX_ECCPOS_ENTRIES: u32 = 64;
pub const UBI_VOL_NUM_AUTO: i32 = -1;
pub const UBI_DEV_NUM_AUTO: i32 = -1;
pub const UBI_MAX_VOLUME_NAME: u32 = 127;
pub const UBI_IOC_MAGIC: u8 = 111u8;
pub const UBI_CTRL_IOC_MAGIC: u8 = 111u8;
pub const UBI_VOL_IOC_MAGIC: u8 = 79u8;
pub const MAX_UBI_MTD_NAME_LEN: u32 = 127;
pub const UBI_MAX_RNVOL: u32 = 32;
pub const MTD_OPS_PLACE_OOB: _bindgen_ty_1 = _bindgen_ty_1::MTD_OPS_PLACE_OOB;
pub const MTD_OPS_AUTO_OOB: _bindgen_ty_1 = _bindgen_ty_1::MTD_OPS_AUTO_OOB;
pub const MTD_OPS_RAW: _bindgen_ty_1 = _bindgen_ty_1::MTD_OPS_RAW;
pub const UBI_DYNAMIC_VOLUME: _bindgen_ty_2 = _bindgen_ty_2::UBI_DYNAMIC_VOLUME;
pub const UBI_STATIC_VOLUME: _bindgen_ty_2 = _bindgen_ty_2::UBI_STATIC_VOLUME;
pub const UBI_VOL_PROP_DIRECT_WRITE: _bindgen_ty_3 = _bindgen_ty_3::UBI_VOL_PROP_DIRECT_WRITE;
pub const UBI_VOL_SKIP_CRC_CHECK_FLG: _bindgen_ty_4 = _bindgen_ty_4::UBI_VOL_SKIP_CRC_CHECK_FLG;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
MTD_OPS_PLACE_OOB = 0,
MTD_OPS_AUTO_OOB = 1,
MTD_OPS_RAW = 2,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mtd_file_modes {
MTD_FILE_MODE_NORMAL = 0,
MTD_FILE_MODE_OTP_FACTORY = 1,
MTD_FILE_MODE_OTP_USER = 2,
MTD_FILE_MODE_RAW = 3,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_2 {
UBI_DYNAMIC_VOLUME = 3,
UBI_STATIC_VOLUME = 4,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_3 {
UBI_VOL_PROP_DIRECT_WRITE = 1,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_4 {
UBI_VOL_SKIP_CRC_CHECK_FLG = 1,
}
