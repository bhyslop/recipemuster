
TRBM_SENTRY = nsproto-sentry
TRBM_BOTTLE = nsproto-bottle

rbm-t.TestRBM.nsproto.mk:
	@echo "Show that we can access internet from sentry"
	podman exec -it $(TRBM_SENTRY) nslookup example.com
	@echo "Show that we cannot access google from bottle"
	! podman exec -it $(TRBM_BOTTLE) nslookup google.com
	@echo "Show that we can access anthropic from bottle"
	podman exec -it $(TRBM_BOTTLE) nslookup anthropic.com
	@echo "PASS"

