subdir('icons')
subdir('resources')

desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)
desktop_file = i18n.merge_file(
  type: 'desktop',
  input: configure_file(
    input: files('@0@.desktop.in.in'.format(base_id)),
    output: '@0@.desktop.in'.format(base_id),
    configuration: desktop_conf
  ),
  output: '@0@.desktop'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)

if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ],
    depends: desktop_file,
  )
endif

metainfo_conf = configuration_data()
metainfo_conf.set('app-id', application_id)
metainfo_conf.set('gettext-package', meson.project_name())
metainfo = i18n.merge_file(
  input: configure_file(
    input: files('@0@.metainfo.xml.in.in'.format(base_id)),
    output: '@0@.metainfo.xml.in'.format(application_id),
    configuration: metainfo_conf
  ),
  output: '@0@.metainfo.xml'.format(application_id),
  install: true,
  install_dir: join_paths(join_paths('share'), 'metainfo'),
  po_dir: join_paths(meson.project_source_root(), 'po')
)

# Validating the metainfo file
appstreamcli = find_program('appstream-util', required: false)
if appstreamcli.found()
  test (
    'Validate metainfo file',
    appstreamcli,
    args: ['validate-relax', '--nonet', join_paths(meson.current_build_dir (), application_id + '.metainfo.xml')]
  )
endif

gschema_conf = configuration_data()
gschema_conf.set('app-id', application_id)
gschema_conf.set('gettext-package', gettext_package)
configure_file(
  input: '@0@.gschema.xml.in'.format(base_id),
  output: '@0@.gschema.xml'.format(application_id),
  configuration: gschema_conf,
  install: true,
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

if glib_compile_schemas.found()
  test(
    'validate-gschema', glib_compile_schemas,
    args: [
      '--strict', '--dry-run', meson.current_build_dir()
    ],
  )
endif
