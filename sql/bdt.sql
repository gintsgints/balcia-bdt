-- name: get_tables ?
--
-- Returns BDT table list
--
-- # Parameters
--
-- param: table_name_mask: &str - mask to search for busi

select * from v_adm_codif_entry where ic LIKE :table_name_mask
                                  and adm_codificator_id = (
        select id from adm_codif_entry where ic = 'AGC_CUSTOM_TABLE_TYPE' and adm_codificator_id = 1000)
