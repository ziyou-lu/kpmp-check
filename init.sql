drop table if exists kpmp_analysis_kdstaticface ;
CREATE TABLE kpmp_analysis_kdstaticface (
                                                   id serial4 NOT NULL,
                                                   faceid varchar(64) NULL,
                                                   shottime timestamptz NULL,
                                                   tabid varchar(128) NULL,
                                                   del int4 NULL,
                                                   featuredata varchar(1024) NULL,
                                                   CONSTRAINT kpmp_analysis_kdstaticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_analysis_kdstaticface_faceid_idx ON kpmp_analysis_kdstaticface USING btree (faceid, shottime);
CREATE INDEX kpmp_analysis_kdstaticface_shottime_idx ON kpmp_analysis_kdstaticface USING btree (shottime);

drop table if exists kpmp_analysis_staticface ;
CREATE TABLE kpmp_analysis_staticface (
                                            id serial4 NOT NULL,
                                            faceid varchar(64) NULL,
                                            shottime timestamptz NULL,
                                            tabid varchar(128) NULL,
                                            del int4 NULL,
                                            CONSTRAINT kpmp_analysis_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_analysis_staticface_faceid_idx ON kpmp_analysis_staticface USING btree (faceid, shottime);
CREATE INDEX kpmp_analysis_staticface_shottime_idx ON kpmp_analysis_staticface USING btree (shottime);