-- name: get_tables ?
-- # Parameters
-- param: ic_code: &str - mask to search for business table

SELECT c.id, c.IC, c.NAME, tp.CODE AS tariff_plan_code, c.VALID_FROM, c.VALID_TO, c.notes, ctt.emails
FROM agc_custom_table_type ctt
    JOIN v_adm_codif_entry c ON c.id = ctt.id
    LEFT JOIN agc_tariff_plan tp ON tp.id = ctt.agc_tariff_plan_id
WHERE
    c.IC LIKE :IC_CODE

-- name: get_table_names ?
-- # Parameters
-- param: ic_code: &str - business table IC code

select cdfl.lng_code, cdfl.name, cdfl.print_name, cdfl.short_print_name from v_adm_codif_entry cdf
left join adm_codif_entry_lng cdfl on cdf.id = cdfl.adm_codif_entry_id
where cdf.ic = :IC_CODE
  and cdf.adm_codificator_id = (
    select id from adm_codif_entry where ic = 'AGC_CUSTOM_TABLE_TYPE' and adm_codificator_id = 1000)

-- name: business_table_column_definition ?
-- # Parameters
-- param: table_ic: &str - business table IC code

select  acc.id, acc.sequence, acc.title,  acc.ref_code, acc.col_name,
        v.ic as codificator_ic, acc.is_key, acc.options, acc.select_params, acc.table_type_id
from  agc_ctb_columns acc
          left join v_adm_codif_entry v on  acc.adm_codificator_id is not null
    and acc.adm_codificator_id = v.id
where acc.table_type_id =
        (select id from v_adm_codif_entry where ic = :TABLE_IC
                                  and adm_codificator_id = (
        select id from adm_codif_entry where ic = 'AGC_CUSTOM_TABLE_TYPE' and adm_codificator_id = 1000))

-- name: business_table_data ?
-- # Parameters
-- param: table_ic: &str - business table IC code

select
    cd.id
     ,f.decode_codif_ic(cd.cdf1_id) AS cdf1_ic
     ,f.decode_codif_ic(cd.cdf2_id) AS cdf2_ic
     ,f.decode_codif_ic(cd.cdf3_id) AS cdf3_ic
     ,f.decode_codif_ic(cd.cdf4_id) AS cdf4_ic
     ,f.decode_codif_ic(cd.cdf5_id) AS cdf5_ic
     ,f.decode_codif_ic(cd.cdf6_id) AS cdf6_ic
     ,f.decode_codif_ic(cd.cdf7_id) AS cdf7_ic
     ,f.decode_codif_ic(cd.cdf8_id) AS cdf8_ic
     ,f.decode_codif_ic(cd.cdf9_id) AS cdf9_ic
     ,f.decode_codif_ic(cd.cdf10_id) AS cdf10_ic
     ,f.decode_codif_ic(cd.cdf11_id) AS cdf11_ic
     ,f.decode_codif_ic(cd.cdf12_id) AS cdf12_ic
     ,f.decode_codif_ic(cd.cdf13_id) AS cdf13_ic
     ,f.decode_codif_ic(cd.cdf14_id) AS cdf14_ic
     ,f.decode_codif_ic(cd.cdf15_id) AS cdf15_ic
     ,f.decode_codif_ic(cd.cdf16_id) AS cdf16_ic
     ,f.decode_codif_ic(cd.cdf17_id) AS cdf17_ic
     ,f.decode_codif_ic(cd.cdf18_id) AS cdf18_ic
     ,f.decode_codif_ic(cd.cdf19_id) AS cdf19_ic
     ,f.decode_codif_ic(cd.cdf20_id) AS cdf20_ic
     ,f.decode_codif_ic(cd.cdf21_id) AS cdf21_ic
     ,f.decode_codif_ic(cd.cdf22_id) AS cdf22_ic
     ,f.decode_codif_ic(cd.cdf23_id) AS cdf23_ic
     ,f.decode_codif_ic(cd.cdf24_id) AS cdf24_ic
     ,f.decode_codif_ic(cd.cdf25_id) AS cdf25_ic

     ,cd.num1
     ,cd.num2
     ,cd.num3
     ,cd.num4
     ,cd.num5
     ,cd.num6
     ,cd.num7
     ,cd.num8
     ,cd.num9
     ,cd.num10
     ,cd.num11
     ,cd.num12
     ,cd.num13
     ,cd.num14
     ,cd.num15
     ,cd.num16
     ,cd.num17
     ,cd.num18
     ,cd.num19
     ,cd.num20

     ,cd.text1
     ,cd.text2
     ,cd.text3
     ,cd.text4
     ,cd.text5
     ,cd.text6
     ,cd.text7
     ,cd.text8
     ,cd.text9
     ,cd.text10
     ,cd.valid_from
     ,cd.valid_to
FROM agc_ctb_data cd
where cd.table_type_id =
      (select id from v_adm_codif_entry where ic = :TABLE_IC
                                          and adm_codificator_id = (
              select id from adm_codif_entry where ic = 'AGC_CUSTOM_TABLE_TYPE' and adm_codificator_id = 1000))
