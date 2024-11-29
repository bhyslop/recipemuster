include mbc.MakefileBashConsole.mk

tmbc_check_domain_test_all:                 \
  ztmbc_check_domain_test_basic             \
  ztmbc_check_domain_test_subdomain         \
  ztmbc_check_domain_test_hyphenated        \
  ztmbc_check_domain_test_numeric           \
  ztmbc_check_domain_test_many_levels       \
  ztmbc_check_domain_test_start_dot         \
  ztmbc_check_domain_test_end_dot           \
  ztmbc_check_domain_test_double_dot        \
  ztmbc_check_domain_test_underscore        \
  ztmbc_check_domain_test_start_hyphen      \
  ztmbc_check_domain_test_end_hyphen        \
  # end-of-list
	$(MBC_PASS) "All domain format tests completed"

ztmbc_check_domain_test_basic:
	@(export THE_DOMAIN=example.com && \
	  echo "Testing basic domain format: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN))
	$(MBC_PASS) "Basic domain test passed"

ztmbc_check_domain_test_subdomain:
	@(export THE_DOMAIN=sub.example.com && \
	  echo "Testing subdomain format: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN))
	$(MBC_PASS) "Subdomain test passed"

ztmbc_check_domain_test_hyphenated:
	@(export THE_DOMAIN=my-domain123.com && \
	  echo "Testing hyphenated domain: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN))
	$(MBC_PASS) "Hyphenated domain test passed"

ztmbc_check_domain_test_numeric:
	@(export THE_DOMAIN=123.456.com && \
	  echo "Testing numeric labels: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN))
	$(MBC_PASS) "Numeric labels test passed"

ztmbc_check_domain_test_many_levels:
	@(export THE_DOMAIN=a.b.c.d.e.f.g.com && \
	  echo "Testing many subdomain levels: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN))
	$(MBC_PASS) "Many levels test passed"

ztmbc_check_domain_test_start_dot:
	@(! (export THE_DOMAIN=.starts-with-dot.com && \
	  echo "Testing domain starting with dot: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN)))
	$(MBC_PASS) "Start dot test passed"

ztmbc_check_domain_test_end_dot:
	@(! (export THE_DOMAIN=ends-with-dot. && \
	  echo "Testing domain ending with dot: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN)))
	$(MBC_PASS) "End dot test passed"

ztmbc_check_domain_test_double_dot:
	@(! (export THE_DOMAIN=double..dot.com && \
	  echo "Testing consecutive dots: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN)))
	$(MBC_PASS) "Double dot test passed"

ztmbc_check_domain_test_underscore:
	@(! (export THE_DOMAIN=under_score.com && \
	  echo "Testing underscore in domain: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN)))
	$(MBC_PASS) "Underscore test passed"

ztmbc_check_domain_test_start_hyphen:
	@(! (export THE_DOMAIN=-starts-with-dash.com && \
	  echo "Testing domain starting with hyphen: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN)))
	$(MBC_PASS) "Start hyphen test passed"

ztmbc_check_domain_test_end_hyphen:
	@(! (export THE_DOMAIN=ends-with-dash-.com && \
	  echo "Testing domain ending with hyphen: $$THE_DOMAIN" && \
	  $(call MBC_CHECK_ISDOMAIN,1,$$THE_DOMAIN)))
	$(MBC_PASS) "End hyphen test passed"

